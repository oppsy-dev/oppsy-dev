import dagger
from dagger import dag, function, object_type, DefaultPath
from typing import Annotated


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
    def backend_build(
        self, src: Annotated[dagger.Directory, DefaultPath(".")]
    ) -> dagger.File:
        return (
            dag.container()
            .from_("rust:1.91-slim")
            .with_exec(["apt-get", "update"])
            .with_exec(["apt-get", "install", "-y", "--no-install-recommends",
                        "cmake", "build-essential", "libsqlite3-dev", "pkg-config", "ca-certificates", "golang"])
            .with_exec(["apt", "install", "golang", "-y"])
            .with_directory("/build", src.directory("backend"))
            .with_workdir("/build")
            .with_exec(["cargo", "build", "--release", "-p", "service"])
            .file("/build/target/release/service")
        )

    @function
    def core_db(
        self, src: Annotated[dagger.Directory, DefaultPath(".")]
    ) -> dagger.Directory:
        core_db_src = src.directory("backend/core-db")
        return (
            dag.directory()
            .with_file("atlas.hcl", core_db_src.file("atlas.hcl"))
            .with_directory("sqlite-migrations", core_db_src.directory("sqlite-migrations"))
        )

    @function
    async def oppsy_build(
        self, src: Annotated[dagger.Directory, DefaultPath(".")]
    ) -> dagger.Container:
        frontend = self.frontend_build(src)
        binary = self.backend_build(src)
        core_db = self.core_db(src)

        return (
            dag.container()
            .from_("debian:trixie-slim")
            .with_exec(["apt-get", "update"])
            .with_exec(["apt-get", "install", "-y", "--no-install-recommends",
                        "ca-certificates", "curl"])
            .with_exec(["apt-get", "clean"])
            .with_exec(["rm", "-rf", "/var/lib/apt/lists/*"])
            .with_exec(["sh", "-c", "curl -sSf https://atlasgo.sh | sh"])
            .with_file("/usr/local/bin/service", binary)
            .with_directory("/core-db", core_db)
            .with_directory("/frontend", frontend)
            .with_env_variable("OSV_SERVICE_FRONTEND_PATH", "/frontend")
            .with_env_variable("OSV_SERVICE_CORE_DB_URL", "sqlite:///data/oppsy.db")
            .with_env_variable("OSV_SERVICE_BIND_ADDRESS", "0.0.0.0:3030")
            .with_exposed_port(3030)
            .with_entrypoint([
                "sh", "-c",
                "atlas migrate apply --url sqlite:///data/oppsy.db --env sqlite --config file:///core-db/atlas.hcl"
                " && exec /usr/local/bin/service",
            ])
        )