backend_manifest := "backend/Cargo.toml"
frontend_dir := "frontend"
openapi_spec := "api/oppsy-openapi.json"

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

# Export the backend OpenAPI schema to api/oppsy-openapi.json
gen-openapi:
    mkdir -p $(dirname {{openapi_spec}})
    cargo run --manifest-path {{backend_manifest}} -p service -- docs > {{openapi_spec}}

# Generate the TypeScript API client from the backend OpenAPI schema
frontend-gen-api-client: gen-openapi
    npx openapi-typescript {{openapi_spec}} -o {{frontend_dir}}/src/api/schema.d.ts

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

# Build the mdBook documentation site (generates OpenAPI spec first)
build-docs: gen-openapi
    mdbook build
    cp {{openapi_spec}} book/oppsy-openapi.json

oppsy-build:
    dagger call oppsy-build --src=. export-image --name oppsy:latest

oppsy-run:
    podman run -d --name oppsy -p 3030:3030 -v oppsy-data:/data oppsy:latest

oppsy-stop:
    podman rm -f oppsy

