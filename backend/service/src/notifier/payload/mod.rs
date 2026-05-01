mod discord;
mod email;
mod webhook;

pub use discord::osv_discord_event_payload;
pub use email::osv_email_event_payload;
pub use webhook::osv_webhook_event_payload;

#[cfg(test)]
mod tests {
    use poem_openapi::types::{Example, ToJSON};

    use crate::types::NotificationEventMeta;

    #[test]
    fn schema_validity_test() {
        let cue_ctx = cue_rs::Ctx::new().unwrap();

        let schema =
            cue_rs::Value::compile_bytes(&cue_ctx, include_bytes!("meta_schema.cue")).unwrap();
        schema.is_valid().unwrap();

        let meta = NotificationEventMeta::example();
        let meta = cue_rs::Value::compile_string(&cue_ctx, meta.to_json_string().as_str()).unwrap();

        let meta = cue_rs::Value::unify(&schema, &meta);
        if let Err(err) = meta.is_valid() {
            println!("{err}");
        }
    }
}
