use notifier::email::EmailEventPayload;

use crate::types::NotificationEventMeta;

pub fn osv_email_event_payload(meta: &NotificationEventMeta) -> EmailEventPayload {
    let vuln_count = meta.osv_records.len();

    let subject = format!(
        "[OPPSY] {vuln_count} new vulnerabilit{} detected in {}/{}",
        if vuln_count == 1 { "y" } else { "ies" },
        meta.workspace_name,
        meta.manifest_name,
    );

    let tag_line = match &meta.manifest_tag {
        Some(tag) => format!("  Tag:            {tag}\n"),
        None => String::new(),
    };

    let vuln_list: String = meta
        .osv_records
        .iter()
        .fold(String::new(), |res, id| format!("{res}  - {id}\n"));

    let body = format!(
        "OPPSY detected {vuln_count} new open-source vulnerabilit{} in your manifest.\n\
        \n\
        Workspace:      {workspace}\n\
        Manifest:       {manifest} ({manifest_type})\n\
        {tag_line}\
        Vulnerabilities ({vuln_count}):\n\
        {vuln_list}\n\
        Review each finding at https://osv.dev and assess whether your project is affected.\n\
        \n\
        --\n\
        To stop receiving these emails, disable or delete this notification channel in OPPSY.",
        if vuln_count == 1 { "y" } else { "ies" },
        workspace = meta.workspace_name,
        manifest = meta.manifest_name,
        manifest_type = meta.manifest_type,
    );

    EmailEventPayload { subject, body }
}
