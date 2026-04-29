// Atlas project configuration.
// Docs: https://atlasgo.io/atlas-schema/projects


env "sqlite" {
  // In-memory SQLite Atlas uses internally to normalise and validate SQL
  // before writing migration files. Never the target of `migrate apply`.
  dev = "sqlite://oppsy-dev?mode=memory"

  migration {
    dir    = "file://sqlite-migrations"
    format = atlas
  }
}
