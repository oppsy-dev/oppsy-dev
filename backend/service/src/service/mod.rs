mod api;
mod common;

use poem::{
    Endpoint, EndpointExt, Route, Server, http,
    listener::TcpListener,
    middleware::{Cors, SensitiveHeader, Tracing},
};
use poem_openapi::{OpenApi, OpenApiService, Webhook};

use crate::{resources::ResourceRegistry, service::api::mk_api, settings::Settings};

/// Returns the `OpenAPI` JSON schema without starting the server.
pub fn spec() -> String {
    mk_api("").spec()
}

pub async fn run() -> anyhow::Result<()> {
    let settings = ResourceRegistry::get::<Settings>()?;
    let app = mk_app(&settings)?;

    Server::new(TcpListener::bind(settings.bind_address))
        .run(app)
        .await?;
    Ok(())
}

fn mk_app(settings: &Settings) -> anyhow::Result<impl Endpoint + use<>> {
    let api = mk_api(&settings.api_url_prefix);
    let docs = docs(&api);

    if !cfg!(feature = "local-dev") && settings.allowed_cors_origins.is_empty() {
        anyhow::bail!(
            "allowed_cors_origins is empty — all origins are permitted by the CORS \
            middleware and open-redirect protection on the auth callback is disabled. \
            This is only acceptable for local development, built with '--features local-dev' flag."
        );
    }

    let cors = if settings.allowed_cors_origins.is_empty() {
        // Dev mode: reflect the request's Origin header and allow credentials.
        // `Access-Control-Allow-Origin: *` is forbidden when credentials are
        // included, so we echo back whatever origin the browser sent instead.
        Cors::new()
            .allow_credentials(true)
            .allow_origins_fn(|_| true)
    } else {
        settings
            .allowed_cors_origins
            .iter()
            .fold(Cors::new().allow_credentials(true), |cors, origin| {
                cors.allow_origin(origin.origin().ascii_serialization().as_str())
            })
    };

    Ok(Route::new()
        .nest(&settings.api_url_prefix, api)
        .nest("/docs", docs)
        .with(Tracing)
        .with(
            SensitiveHeader::new()
                .header(http::header::COOKIE)
                .header(http::header::SET_COOKIE),
        )
        .with(cors))
}

fn docs(api: &OpenApiService<impl OpenApi + 'static, impl Webhook + 'static>) -> Route {
    let swagger_ui = api.swagger_ui();
    let rapidoc_ui = api.rapidoc();
    let redoc_ui = api.redoc();
    let openapi_explorer = api.openapi_explorer();

    Route::new()
        .nest("/", swagger_ui)
        .nest("/redoc", redoc_ui)
        .nest("/rapidoc", rapidoc_ui)
        .nest("/openapi_explorer", openapi_explorer)
        .at("/open-api.json", api.spec_endpoint())
}
