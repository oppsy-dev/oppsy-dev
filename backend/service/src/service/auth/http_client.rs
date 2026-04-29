use std::pin::Pin;

/// `reqwest` [`Client`](reqwest::Client) thin wrapper to implement
/// [`oauth2::AsyncHttpClient`].
pub struct HttpClient(reqwest::Client);

impl HttpClient {
    pub fn new() -> Self {
        Self(reqwest::Client::new())
    }
}

impl<'c> oauth2::AsyncHttpClient<'c> for HttpClient {
    type Error = oauth2::HttpClientError<reqwest::Error>;
    type Future =
        Pin<Box<dyn Future<Output = Result<oauth2::HttpResponse, Self::Error>> + Send + Sync + 'c>>;

    fn call(
        &'c self,
        request: oauth2::HttpRequest,
    ) -> Self::Future {
        Box::pin(async move {
            let response = self
                .0
                .execute(request.try_into().map_err(Box::new)?)
                .await
                .map_err(Box::new)?;

            let mut builder = oauth2::http::Response::builder().status(response.status());

            builder = builder.version(response.version());

            for (name, value) in response.headers() {
                builder = builder.header(name, value);
            }

            builder
                .body(response.bytes().await.map_err(Box::new)?.to_vec())
                .map_err(oauth2::HttpClientError::Http)
        })
    }
}
