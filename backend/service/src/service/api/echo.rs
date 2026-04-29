use poem_openapi::{OpenApi, param::Query, payload::PlainText};

pub struct Api;

#[OpenApi]
impl Api {
    /// Returns the provided message as-is.
    #[oai(path = "/echo", method = "get")]
    #[allow(clippy::unused_async)]
    async fn echo(
        &self,
        message: Query<String>,
    ) -> PlainText<String> {
        PlainText(message.0)
    }
}
