use std::collections::HashMap;

use rust_embed::RustEmbed;
use serde::Deserialize;

/// Describes a deprecated API version (group/version)
///
/// Each `DeprecatedVersion` contains the deprecated API in the `group/version` format
/// the kind, the Kubernetes versions where it was deprecated and removed, and
/// it may or may not provide a replacement API version (if there is one)
#[derive(Deserialize, Debug, Clone)]
pub struct DeprecatedVersion {
    /// The API version in `group/version` format
    pub api_version: String,
    /// Kind of the object associated with this version
    pub kind: String,
    /// The version of Kubernetes where API was initially marked as deprecated
    pub deprecated_in: String,
    /// The version of Kubernetes where the API was finally removed
    pub removed_in: String,
    /// The replacement API version, if one is available
    pub replacement_api_version: Option<String>,
}

/// Represents a group/version/kind
pub type GroupVersionKind = String;

/// Contains the deprecated API versions
///
/// `Deprecated` holds a map of `GroupVersionKind`s (GVK) mapped to their repspective
/// `DeprecatedVersion` struct for quick lookup to check if a GVK is in the map of
/// deprecated versions or not
#[derive(Deserialize, Debug)]
pub struct Deprecated {
    /// Map of GroupVersionKind mapped to its respective deprecated version struct
    pub versions: HashMap<GroupVersionKind, DeprecatedVersion>,
}

/// Contains the static map of deprecated API versions in YAML format
#[derive(RustEmbed)]
#[folder = "data/"]
struct Data;

/// Builds the map of `GroupVersionKinds` mapped to their respective `DeprecatedVersion` struct
impl Deprecated {
    pub fn get() -> Result<Self, anyhow::Error> {
        let deprecation_file = Data::get("deprecations.yaml").unwrap();
        let contents = std::str::from_utf8(deprecation_file.data.as_ref()).unwrap();
        let data: Vec<DeprecatedVersion> = serde_yaml::from_str(contents)?;

        let mut versions: HashMap<GroupVersionKind, DeprecatedVersion> = HashMap::new();
        for d in data.iter() {
            let gvk = format!("{}/{}", d.api_version, d.kind);
            versions.insert(gvk, d.clone());
        }

        Ok(Deprecated { versions })
    }
}
