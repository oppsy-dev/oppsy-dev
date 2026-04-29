use poem_openapi::{NewType, types::Example};

use crate::types::uuid_v7::UuidV7;

/// UUID v7 identifier for a notification channel.
#[derive(NewType, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[oai(example)]
pub struct NotificationChannelId(UuidV7);

impl NotificationChannelId {
    /// Generates a new random notification channel ID.
    pub fn generate() -> Self {
        Self(UuidV7::generate())
    }
}

impl Example for NotificationChannelId {
    fn example() -> Self {
        Self(UuidV7::example())
    }
}

impl std::fmt::Display for NotificationChannelId {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<NotificationChannelId> for core_db::notification_channel::NotificationChannelId {
    fn from(value: NotificationChannelId) -> Self {
        value.0.into()
    }
}

impl TryFrom<core_db::notification_channel::NotificationChannelId> for NotificationChannelId {
    type Error = <UuidV7 as TryFrom<uuid::Uuid>>::Error;

    fn try_from(
        value: core_db::notification_channel::NotificationChannelId
    ) -> Result<Self, Self::Error> {
        value.try_into().map(Self)
    }
}
