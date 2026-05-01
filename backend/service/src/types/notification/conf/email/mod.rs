use notifier::email::EmailEventPayload;
use poem_openapi::{Object, types::Example};

use crate::types::{NotificationEventMeta, email::EmailAddress};

/// Configuration for an email notification channel.
#[derive(Debug, Clone, Object)]
#[oai(example)]
pub struct EmailChannelConf {
    /// Sender email address
    pub from: EmailAddress,
    /// Recipient email addresses.
    pub to: Vec<EmailAddress>,
    /// Email event payload template, as a CUE encoded string
    pub template: String,
}

impl EmailChannelConf {
    pub const TEMPLATE_SCHEMA: &[u8] = include_bytes!("schema.cue");

    pub fn event_payload(
        &self,
        cue_ctx: &cue_rs::Ctx,
        meta: &NotificationEventMeta,
    ) -> anyhow::Result<EmailEventPayload> {
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

impl Example for EmailChannelConf {
    fn example() -> Self {
        Self {
            from: EmailAddress::example(),
            to: vec![EmailAddress::example()],
            // TODO: proper template value
            template: String::new(),
        }
    }
}

impl From<EmailChannelConf> for notifier::email::EmailEventConfig {
    fn from(value: EmailChannelConf) -> Self {
        Self {
            from: value.from.into(),
            to: value.to.into_iter().map(Into::into).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn payload() {
        let cue_ctx = cue_rs::Ctx::new().unwrap();

        let mut conf = EmailChannelConf::example();
        let meta = &NotificationEventMeta::example();

        conf.template = r#"body: "Some body", subject: "Some subject""#.to_string();
        let payload = conf.event_payload(&cue_ctx, meta).unwrap();
        assert_eq!(payload, EmailEventPayload {
            subject: "Some subject".to_string(),
            body: "Some body".to_string(),
        });

        conf.template = r#"body: "Some body with \(_manifest_name)", subject: "Some subject with \(_workspace_name)""#.to_string();
        let payload = conf.event_payload(&cue_ctx, meta).unwrap();
        assert_eq!(payload, EmailEventPayload {
            subject: format!("Some subject with {}", meta.workspace_name),
            body: format!("Some body with {}", meta.manifest_name),
        });
    }
}
