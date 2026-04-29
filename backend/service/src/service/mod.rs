mod api;
mod common;

use poem::{
    Endpoint, EndpointExt, Route, Server,
    endpoint::StaticFilesEndpoint,
    http,
    listener::TcpListener,
    middleware::{SensitiveHeader, Tracing},
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

#[allow(clippy::unnecessary_wraps)]
fn mk_app(settings: &Settings) -> anyhow::Result<impl Endpoint + use<>> {
    let api = mk_api(&settings.api_url_prefix);
    let docs = docs(&api);

    let static_files = StaticFilesEndpoint::new(&settings.frontend_path)
        .index_file("index.html")
        .fallback_to_index();

    Ok(Route::new()
        .nest(&settings.api_url_prefix, api)
        .nest("/docs", docs)
        .nest("/", static_files)
        .with(Tracing)
        .with(
            SensitiveHeader::new()
                .header(http::header::COOKIE)
                .header(http::header::SET_COOKIE),
        ))
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
