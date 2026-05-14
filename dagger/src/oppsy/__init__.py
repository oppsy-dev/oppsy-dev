from typing import Annotated

import dagger
from dagger import DefaultPath, dag, function, object_type


@object_type
class Oppsy:
    @function
    def frontend_build(
        self, src: Annotated[dagger.Directory, DefaultPath(".")]
    ) -> dagger.Directory:
        return (
            dag.container()
            .from_("node:20-alpine")
            .with_directory("/app", src.directory("frontend"))
            .with_workdir("/app")
            .with_exec(["yarn", "install", "--frozen-lockfile"])
            .with_exec(["yarn", "build"])
            .directory("/app/build")
        )

    @function
    def backend_build(self, src: Annotated[dagger.Directory, DefaultPath(".")]) -> dagger.Directory:
        return (
            dag.container()
            .from_("rust:1.91-slim")
            .with_exec(["apt-get", "update"])
            .with_exec(
                [
                    "apt-get",
                    "install",
                    "-y",
                    "--no-install-recommends",
                    "cmake",
                    "build-essential",
                    "libsqlite3-dev",
                    "pkg-config",
                    "ca-certificates",
                    "golang",
                ]
            )
            .with_directory("/build", src.directory("backend"))
            .with_workdir("/build")
            .with_exec(["cargo", "build", "--locked", "--release", "-p", "service"])
            # leave only `service`, `libosv-scalibr.so`, `libcue.so`
            .without_directory("/build/target/release/build")
            .without_directory("/build/target/release/deps")
            .without_directory("/build/target/release/examples")
            .without_directory("/build/target/release/incremental")
            .directory("/build/target/release")
        )

    @function
    def core_db(self, src: Annotated[dagger.Directory, DefaultPath(".")]) -> dagger.Container:
        core_db_src = src.directory("backend/core-db")
        return (
            dag.container()
            .from_("debian:trixie-slim")
            .with_exec(["apt-get", "update"])
            .with_exec(
                [
                    "apt-get",
                    "install",
                    "-y",
                    "--no-install-recommends",
                    "ca-certificates",
                    "curl",
                ]
            )
            .with_exec(["apt-get", "clean"])
            .with_exec(["rm", "-rf", "/var/lib/apt/lists/*"])
            .with_exec(["sh", "-c", "curl -sSf https://atlasgo.sh | sh"])
            .with_directory(
                "/core-db",
                dag.directory()
                .with_file("atlas.hcl", core_db_src.file("atlas.hcl"))
                .with_directory("sqlite-migrations", core_db_src.directory("sqlite-migrations")),
            )
        )

    @function
    async def oppsy_build(
        self, src: Annotated[dagger.Directory, DefaultPath(".")]
    ) -> dagger.Container:
        frontend = self.frontend_build(src)
        backend = self.backend_build(src)
        core_db = self.core_db(src)

        return (
            dag.container()
            .from_("debian:trixie-slim")
            .with_exec(["apt-get", "update"])
            .with_exec(
                [
                    "apt-get",
                    "install",
                    "-y",
                    "--no-install-recommends",
                    "ca-certificates",
                ]
            )
            .with_exec(["apt-get", "clean"])
            .with_exec(["rm", "-rf", "/var/lib/apt/lists/*"])
            .with_file("/usr/local/bin/atlas", core_db.file("/usr/local/bin/atlas"))
            .with_directory("/usr/local/bin", backend)
            # libcue.so and libosv-scalibr.so land in /usr/local/bin alongside the
            # service binary, but the dynamic linker never searches /usr/local/bin.
            # TODO: proper fix is to register the path via ldconfig:
            #   echo '/usr/local/bin' > /etc/ld.so.conf.d/service.conf && ldconfig
            .with_env_variable("LD_LIBRARY_PATH", "/usr/local/bin")
            .with_directory("/data/core-db", core_db.directory("/core-db"))
            .with_directory("/frontend", frontend)
            .with_file(
                "/entrypoint.sh",
                src.file("dagger/scripts/entrypoint.sh"),
                permissions=0o755,
            )
            .with_env_variable("OPPSY_SERVICE_FRONTEND_PATH", "/frontend")
            .with_env_variable("OPPSY_SERVICE_CORE_DB_URL", "sqlite:///data/core-db/oppsy.db")
            .with_env_variable("OPPSY_SERVICE_MANIFEST_DB_PATH", "/data/manifest-db")
            .with_env_variable("OPPSY_SERVICE_OSV_DB_PATH", "/data/osv-db")
            .with_env_variable("OPPSY_SERVICE_BIND_ADDRESS", "0.0.0.0:3030")
            .with_exposed_port(3030)
            .with_entrypoint(["/entrypoint.sh"])
        )

    @function
    def cli_build(
        self,
        src: Annotated[dagger.Directory, DefaultPath(".")],
        version: str,
        goos: str,
        goarch: str,
    ) -> dagger.File:
        """Compiles oppsy-cli for the given GOOS/GOARCH and returns the archive
        (.tar.gz for Linux/macOS, .zip for Windows) as a single File."""
        ext = ".exe" if goos == "windows" else ""
        binary = f"oppsy-cli{ext}"
        archive_base = f"oppsy-cli_{version}_{goos}_{goarch}"

        built = (
            dag.container()
            .from_("golang:1.24-alpine")
            .with_exec(["apk", "add", "--no-cache", "tar", "zip"])
            .with_directory("/src", src.directory("oppsy-cli"))
            .with_workdir("/src")
            .with_exec(["mkdir", "-p", "/out", "/archives"])
            .with_env_variable("GOOS", goos)
            .with_env_variable("GOARCH", goarch)
            .with_env_variable("CGO_ENABLED", "0")
            .with_exec(["go", "build", "-o", f"/out/{binary}", "."])
        )

        if goos == "windows":
            return built.with_exec(
                [
                    "zip",
                    "-j",
                    f"/archives/{archive_base}.zip",
                    f"/out/{binary}",
                ]
            ).file(f"/archives/{archive_base}.zip")
        else:
            return built.with_exec(
                [
                    "tar",
                    "czf",
                    f"/archives/{archive_base}.tar.gz",
                    "-C",
                    "/out",
                    binary,
                ]
            ).file(f"/archives/{archive_base}.tar.gz")

    @function
    async def oppsy_publish(
        self,
        src: Annotated[dagger.Directory, DefaultPath(".")],
        address: str,
        username: str,
        secret: dagger.Secret,
    ) -> str:
        container = await self.oppsy_build(src)
        return await container.with_registry_auth("ghcr.io", username, secret).publish(address)
