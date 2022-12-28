use anyhow::*;
use rust_embed::RustEmbed;
use serde::Deserialize;

// A version holds the information about a deprecated API version and potential replacement API
#[derive(Deserialize, Debug)]
pub struct Version {
    // Name of the API version
    pub api_version: String,
    // Kind of the object associated with this version
    pub kind: String,
    // DeprecatedIn indicates what version the API is deprecated in
    // an empty string indicates that the version is not deprecated
    pub deprecated_in: Option<String>,
    // RemovedIn denotes the version that the api was actually removed in
    // `None` indicates that the version has not been removed yet
    pub removed_in: Option<String>,
    // ReplacementAPI is the apiVersion that replaces the deprecated one
    pub replacement_api_version: Option<String>,
}

// Holds the deprecated API versions
#[derive(Deserialize, Debug)]
pub struct Deprecations {
    pub deprecations: Vec<Version>,
}

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

impl Deprecations {
    pub fn new() -> Result<Self, anyhow::Error> {
        let deprecation_file = Assets::get("deprecation.yaml").unwrap();
        let contents = std::str::from_utf8(deprecation_file.data.as_ref()).unwrap();

        let deprecations: Deprecations = serde_yaml::from_str(contents)?;

        Ok(deprecations)
    }
}
