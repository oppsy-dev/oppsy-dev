use notifier::email::EmailEventPayload;

use crate::types::NotificationEventMeta;

pub fn osv_email_event_payload(_meta: NotificationEventMeta) -> EmailEventPayload {
    EmailEventPayload {
        subject: "".to_string(),
        body: "".to_string(),
    }
}
