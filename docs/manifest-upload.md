# Manifest upload

A manifest is a dependency lock file attached to a workspace. Once uploaded, OPPSY parses it, identifies every package it declares, and runs a vulnerability scan against the OSV database.

## Via the UI

1. Open a workspace and go to the **Manifests** tab.
2. Click **Upload manifest**.
3. Choose the ecosystem (`manifest_type`) that matches your lock file.
4. Give the manifest a **name** — typically the file path or the repo it came from (e.g. `backend/Cargo.lock`).
5. Optionally add a **tag** to distinguish environments or releases (e.g. `production`, `v2.1.0`).
6. Select the lock file from disk and confirm.

OPPSY stores the file and runs the vulnerability scan immediately.

## Via the REST API

Uploading a manifest is a two-step process: first reserve a slot to get an ID, then send the file bytes.

**Step 1 — reserve a manifest slot:**

```bash
curl -s -X POST http://localhost:3030/v1/workspaces/{workspace_id}/manifests \
  -H "Content-Type: application/json" \
  -d '{"manifest_type": "Cargo", "name": "backend/Cargo.lock"}' \
  | jq -r '.'
# returns the manifest_id, e.g. "01926f4e-2a1b-7c3d-9e5f-0a1b2c3d4e5f"
```

**Step 2 — upload the raw file bytes:**

```bash
curl -X PUT http://localhost:3030/v1/workspaces/{workspace_id}/manifests/{manifest_id} \
  -H "Content-Type: application/octet-stream" \
  --data-binary @Cargo.lock
```

A `204 No Content` response means the file was stored and the vulnerability scan has been scheduled.

For full request/response schema details, parameters, and error codes see the [API Reference](./api-reference.html) — look for the **Manifests** tag.
