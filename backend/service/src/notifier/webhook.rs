use crate::types::NotificationEventMeta;

pub fn osv_webhook_event_payload(meta: &NotificationEventMeta) -> serde_json::Value {
    serde_json::json!({
        "event": "osv.vulnerabilities_detected",
        "workspace": {
            "id": meta.workspace_id.to_string(),
            "name": meta.workspace_name.to_string(),
        },
        "manifest": {
            "id": meta.manifest_id.to_string(),
            "name": meta.manifest_name.to_string(),
            "type": meta.manifest_type.to_string(),
            "tag": meta.manifest_tag.as_ref().map(ToString::to_string),
        },
        "vulnerabilities": {
            "count": meta.osv_records.len(),
            "ids": meta.osv_records.iter().map(ToString::to_string).collect::<Vec<_>>(),
        },
    })
}
