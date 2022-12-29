use std::collections::HashMap;

use kube::{api::ApiResource, discovery, Client};
use serde::Deserialize;

/// Represents a group/version/kind
pub type GroupVersionKind = String;

// Holds the deprecated API versions
#[derive(Deserialize, Debug)]
pub struct Discovery {
    pub versions: HashMap<GroupVersionKind, ApiResource>,
}

impl Discovery {
    pub async fn get(client: &Client) -> Result<Self, anyhow::Error> {
        let mut versions: HashMap<GroupVersionKind, ApiResource> = HashMap::new();

        let discovery = discovery::Discovery::new(client.clone()).run().await?;
        for group in discovery.groups() {
            // // This returns ALL of the versions of the resources
            // let vr = group.versioned_resources(group.preferred_version_or_latest());

            for (resource, _capabilities) in group.recommended_resources() {
                // if resource.kind.eq("CSINode") {
                //     println!("{:#?}", resource);
                // }

                let res = resource.clone();
                let api_version = if group.name() == "" {
                    // "core" group does not have a group name, its left blank and only the version is used
                    resource.version
                } else {
                    format!("{}/{}", group.name(), resource.version)
                };

                versions.insert(format!("{api_version}/{}", resource.kind), res);
            }
        }

        Ok(Discovery { versions })
    }
}
