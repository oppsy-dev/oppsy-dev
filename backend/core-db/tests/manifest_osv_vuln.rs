mod common;

use core_db::manifest_osv_vuln::{
    OsvVuln,
    errors::{AddManifestOsvVulnError, GetManifestOsvVulnsError},
};

#[test_with::file(oppsy.db)]
#[tokio::test]
async fn manifest_osv_vuln_roundtrip() {
    let db = common::init_db().await;

    let manifest_id = uuid::Uuid::now_v7();
    db.add_manifest(manifest_id, "cargo", "Cargo.lock", None)
        .await
        .unwrap();

    // Non-existent manifest returns ManifestNotFound
    assert!(matches!(
        db.get_manifest_osv_vulns(uuid::Uuid::now_v7())
            .await
            .unwrap_err(),
        GetManifestOsvVulnsError::ManifestNotFound { .. }
    ));

    // Fresh manifest has no vulnerabilities
    assert!(
        db.get_manifest_osv_vulns(manifest_id)
            .await
            .unwrap()
            .is_empty()
    );

    // Insert multiple vulnerabilities
    let vuln = OsvVuln {
        osv_id: "GHSA-aaaa-bbbb-cccc".to_string(),
        detected_at: 1_700_000_000,
    };

    db.add_manifest_osv_vuln(
        manifest_id,
        [vuln.osv_id.clone()].into_iter().collect(),
        vuln.detected_at,
    )
    .await
    .unwrap();

    // Duplicate is rejected
    assert!(matches!(
        db.add_manifest_osv_vuln(
            manifest_id,
            [vuln.osv_id.clone()].into_iter().collect(),
            vuln.detected_at
        )
        .await
        .unwrap_err(),
        AddManifestOsvVulnError::AlreadyExists { .. }
    ));

    // All inserted vulnerabilities are returned
    let vulns = db.get_manifest_osv_vulns(manifest_id).await.unwrap();
    assert_eq!(vulns, vec![vuln]);
}
