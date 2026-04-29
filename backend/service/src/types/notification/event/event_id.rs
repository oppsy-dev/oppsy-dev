use poem_openapi::{NewType, types::Example};

use crate::types::uuid_v7::UuidV7;

/// UUID v7 identifier for a notification event.
#[derive(NewType, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[oai(example)]
pub struct NotificationEventId(UuidV7);

impl NotificationEventId {
    /// Generates a new random notification event ID.
    pub fn generate() -> Self {
        Self(UuidV7::generate())
    }
}

impl Example for NotificationEventId {
    fn example() -> Self {
        Self(UuidV7::example())
    }
}

impl std::fmt::Display for NotificationEventId {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<NotificationEventId> for core_db::notification_event::NotificationEventId {
    fn from(value: NotificationEventId) -> Self {
        value.0.into()
    }
}

impl TryFrom<core_db::notification_event::NotificationEventId> for NotificationEventId {
    type Error = <UuidV7 as TryFrom<uuid::Uuid>>::Error;

    fn try_from(
        value: core_db::notification_event::NotificationEventId
    ) -> Result<Self, Self::Error> {
        value.try_into().map(Self)
    }
}
