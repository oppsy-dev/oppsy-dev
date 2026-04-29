mod channels;
mod echo;
mod health;
mod osv;
mod workspaces;

use poem_openapi::{OpenApi, OpenApiService, Webhook};

const API_VERSION: &str = "0.1.0";
const API_TITLE: &str = "OPPSY API";

pub fn mk_api(url_prefix: &str) -> OpenApiService<impl OpenApi + 'static, impl Webhook + 'static> {
    OpenApiService::new(
        (
            echo::Api,
            health::Api,
            osv::Api,
            workspaces::Api,
            channels::Api,
        ),
        API_TITLE,
        API_VERSION,
    )
    .url_prefix(url_prefix)
}
