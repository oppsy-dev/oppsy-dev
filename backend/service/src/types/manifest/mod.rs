use core_db::manifest::ManifestName;
use poem_openapi::Object;

use crate::types::{ManifestPackage, ManifestTag};

pub mod id;
pub mod info;
pub mod name;
pub mod package;
pub mod tag;

#[derive(Object, serde::Serialize, serde::Deserialize)]
pub struct Manifest {
    /// Human-readable name for this manifest (e.g. the filename or repo path).
    pub name: ManifestName,
    /// Optional label for versioning or environment disambiguation.
    pub tag: Option<ManifestTag>,
    /// List of manifest's dependencies packages
    pub packages: Vec<ManifestPackage>,
}
