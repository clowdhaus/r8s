use kube::{discovery::Discovery, Client};

use rust_embed::RustEmbed;
use serde::Deserialize;

// Holds the information about a current API version from the discovery API
#[derive(Deserialize, Debug)]
pub struct DiscoveryVersion {
    // Name of the API version
    pub api_version: String,
    // Kind of the object associated with this version
    pub kind: String,
}

// Holds the information about a deprecated API version and potential replacement API
#[derive(Deserialize, Debug)]
pub struct DeprecatedVersion {
    // Name of the API version
    pub api_version: String,
    // Kind of the object associated with this version
    pub kind: String,
    // DeprecatedIn indicates what version the API is deprecated in
    // an empty string indicates that the version is not deprecated
    pub deprecated_in: String,
    // RemovedIn denotes the version that the api was actually removed in
    // `None` indicates that the version has not been removed yet
    pub removed_in: String,
    // ReplacementAPI is the apiVersion that replaces the deprecated one
    pub replacement_api_version: Option<String>,
}

// Holds the deprecated API versions
#[derive(Deserialize, Debug)]
pub struct Deprecated {
    pub versions: Vec<DeprecatedVersion>,
}

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

impl Deprecated {
    pub fn get() -> Result<Self, anyhow::Error> {
        let deprecation_file = Assets::get("deprecations.yaml").unwrap();
        let contents = std::str::from_utf8(deprecation_file.data.as_ref()).unwrap();
        let deprecations: Deprecated = serde_yaml::from_str(contents)?;

        Ok(deprecations)
    }
}

// Holds the deprecated API versions
#[derive(Deserialize, Debug)]
pub struct Cluster {
    pub versions: Vec<DiscoveryVersion>,
}

impl Cluster {
    pub async fn get(client: &Client) -> Result<Self, anyhow::Error> {
        let mut versions = Vec::new();

        let discovery = Discovery::new(client.clone()).run().await?;
        for group in discovery.groups() {
            for (res, _caps) in group.recommended_resources() {
                // if !caps.supports_operation(verbs::LIST) {
                //     continue;
                // }
                // let api: Api<DynamicObject> = if caps.scope == Scope::Cluster {
                //     Api::all_with(client.clone(), &ar)
                // } else {
                //     Api::default_namespaced_with(client.clone(), &ar)
                // };

                let api_version = if group.name() == "" {
                    // "core" group does not have a group name, its left blank and only the version is used
                    res.version
                } else {
                    format!("{}/{}", group.name(), res.version)
                };

                versions.push(DiscoveryVersion {
                    api_version,
                    kind: res.kind,
                })

                // let list = api.list(&Default::default()).await?;
                // for item in list.items {
                //     let name = item.name_any();
                //     let ns = item.metadata.namespace.map(|s| s + "/").unwrap_or_default();
                //     println!("\t\t{}{}", ns, name);
                // }
            }
        }

        Ok(Cluster { versions })
    }
}
