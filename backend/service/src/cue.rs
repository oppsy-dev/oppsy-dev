/// Build a CUE value from a JSON object by mapping each key to a hidden
/// (underscore-prefixed) CUE field, then unifying with the provided schema.
///
/// JSON values are valid CUE literals (strings, numbers, null, arrays, objects),
/// so they can be embedded in a CUE source string directly.
pub fn json_object_to_hidden_fields(
    cue_ctx: &cue_rs::Ctx,
    json: &serde_json::Value,
) -> anyhow::Result<cue_rs::Value> {
    let obj = json
        .as_object()
        .ok_or_else(|| anyhow::anyhow!("expected a JSON object"))?;

    let cue_str = obj
        .iter()
        .map(|(k, v)| format!("_{k}: {v}"))
        .collect::<Vec<_>>()
        .join("\n");

    let value = cue_rs::Value::compile_string(cue_ctx, &cue_str)?;
    value.is_valid()?;
    Ok(value)
}
