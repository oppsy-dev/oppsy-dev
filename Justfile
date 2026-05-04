backend_manifest := "backend/Cargo.toml"
frontend_dir := "frontend"

# List available recipes
default:
    @just --list

# Auto-fix formatting for Rust and Python sources
backend-lint-fix:
    cargo +nightly fmt --manifest-path {{backend_manifest}} --all -- --check

# Check formatting and lints for Rust and Python sources (no writes)
backend-lint-check:
    cargo +nightly fmt --manifest-path {{backend_manifest}} --all -- --check
    cargo +nightly clippy --manifest-path {{backend_manifest}} --all-targets --all-features -- -D warnings
    cargo deny --manifest-path {{backend_manifest}} check

# Generate the TypeScript API client from the backend OpenAPI schema
frontend-gen-api-client:
    cargo run --manifest-path {{backend_manifest}} -p service -- docs > /tmp/oppsy-openapi.json
    npx openapi-typescript /tmp/oppsy-openapi.json -o {{frontend_dir}}/src/api/schema.d.ts

frontend-build-dev: frontend-gen-api-client frontend-lint-fix
    cd {{frontend_dir}} && yarn build

# Auto-fix formatting for frontend sources
frontend-lint-fix:
    cd {{frontend_dir}} && yarn format

# Check formatting for frontend sources (no writes)
frontend-lint-check:
    cd {{frontend_dir}} && yarn prettier --check "src/**/*.{ts,tsx,css}"
    cd {{frontend_dir}} && yarn build

# Format, lint, unit tests, integration tests — run before committing
dev: backend-lint-check frontend-lint-check backend-unit-tests

# Run unit tests
backend-unit-tests:
    cargo test --manifest-path {{backend_manifest}} --all-targets --locked

oppsy-build:
    dagger call oppsy-build --src=. export-image --name oppsy:latest

oppsy-run:
    docker run -d --name oppsy -p 3030:3030 -v oppsy-data:/data oppsy:latest

oppsy-stop:
    docker rm -f oppsy

