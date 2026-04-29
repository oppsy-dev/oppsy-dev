use poem_openapi::Enum;
use strum::{Display, EnumString};

/// The delivery channel used for a notification.
#[derive(Debug, Enum, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString)]
pub enum NotificationChannelType {
    /// HTTP POST webhook with optional HMAC-SHA256 payload signing.
    Webhook,
    /// Discord incoming webhook.
    Discord,
    /// SMTP email delivery to one or more recipients.
    Email,
}
