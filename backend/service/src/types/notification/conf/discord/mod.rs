use notifier::discord::DiscordEventPayload;
use poem_openapi::{Object, types::Example};
use url::Url;

use crate::types::NotificationEventMeta;

/// Configuration for a Discord notification channel.
#[derive(Debug, Clone, Object)]
#[oai(example)]
pub struct DiscordChannelConf {
    /// Discord incoming webhook URL.
    pub discord_webhook_url: Url,
    /// Discord event payload template, as a CUE encoded string
    pub template: String,
}

impl DiscordChannelConf {
    pub const TEMPLATE_SCHEMA: &[u8] = include_bytes!("schema.cue");

    pub fn event_payload(
        &self,
        cue_ctx: &cue_rs::Ctx,
        meta: &NotificationEventMeta,
    ) -> anyhow::Result<DiscordEventPayload> {
        let schema = cue_rs::Value::compile_bytes(cue_ctx, Self::TEMPLATE_SCHEMA)?;
        schema.is_valid()?;

        // Combine with [`NotificationEventMeta::SCHEMA`] so the template compiles in a scope
        // where all meta field types (e.g. manifest_name, workspace_name) are defined
        // alongside the output field constraints.
        let template_bytes = [
            NotificationEventMeta::SCHEMA,
            b"\n",
            self.template.as_bytes(),
        ]
        .concat();

        let template = cue_rs::Value::compile_bytes(cue_ctx, &template_bytes)?;
        let template = cue_rs::Value::unify(&schema, &template);
        let meta_cue = meta.to_cue(cue_ctx)?;
        let payload = cue_rs::Value::unify(&meta_cue, &template);
        payload.is_valid()?;

        let payload_bytes = payload.to_json_bytes()?;
        let payload = serde_json::from_slice(&payload_bytes)?;
        Ok(payload)
    }
}

impl Example for DiscordChannelConf {
    fn example() -> Self {
        #[allow(clippy::unwrap_used)]
        Self {
            discord_webhook_url: Url::parse("https://example.com/webhooks/oppsy").unwrap(),
            // TODO: proper template value
            template: String::new(),
        }
    }
}

impl From<DiscordChannelConf> for notifier::discord::DiscordEventConf {
    fn from(value: DiscordChannelConf) -> Self {
        Self {
            url: value.discord_webhook_url,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn payload() {
        let cue_ctx = cue_rs::Ctx::new().unwrap();

        let mut conf = DiscordChannelConf::example();
        let meta = &NotificationEventMeta::example();

        conf.template = r#"content: "Some content""#.to_string();
        let payload = conf.event_payload(&cue_ctx, meta).unwrap();
        assert_eq!(payload, DiscordEventPayload {
            content: "Some content".to_string(),
        });

        conf.template = r#"content: "Some content with \(_workspace_name)""#.to_string();
        let payload = conf.event_payload(&cue_ctx, meta).unwrap();
        assert_eq!(payload, DiscordEventPayload {
            content: format!("Some content with {}", meta.workspace_name),
        });
    }
}
