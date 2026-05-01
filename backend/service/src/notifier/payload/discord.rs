use notifier::discord::DiscordEventPayload;

use crate::types::NotificationEventMeta;

pub fn osv_discord_event_payload(meta: &NotificationEventMeta) -> DiscordEventPayload {
    let vuln_count = meta.osv_records.len();

    let tag_line = match &meta.manifest_tag {
        Some(tag) => format!("\n**Tag:** `{tag}`"),
        None => String::new(),
    };

    let content = format!(
        ":shield: **OPPSY** detected **{vuln_count}** new open-source vulnerabilit{}\n\n\
        **Workspace:** {workspace}\n\
        **Manifest:** `{manifest}` ({manifest_type}){tag_line}\n\n\
        *To stop receiving these notifications, disable or delete this channel in OPPSY.*",
        if vuln_count == 1 { "y" } else { "ies" },
        workspace = meta.workspace_name,
        manifest = meta.manifest_name,
        manifest_type = meta.manifest_type,
    );

    DiscordEventPayload { content }
}
