mod deprecation;

use anyhow::*;

// use kube::{
//     api::{Api, DynamicObject, ResourceExt},
//     discovery::{verbs, Discovery, Scope},
//     Client,
// };

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let deprecations = deprecation::Deprecations::new()?;
    println!("{deprecations:#?}");

    // let client = Client::try_default().await?;

    // let discovery = Discovery::new(client.clone()).run().await?;
    // for group in discovery.groups() {
    //     for (ar, caps) in group.recommended_resources() {
    //         if !caps.supports_operation(verbs::LIST) {
    //             continue;
    //         }
    //         let api: Api<DynamicObject> = if caps.scope == Scope::Cluster {
    //             Api::all_with(client.clone(), &ar)
    //         } else {
    //             Api::default_namespaced_with(client.clone(), &ar)
    //         };

    //         println!("{}/{} : {}", group.name(), ar.version, ar.kind);

    //         let list = api.list(&Default::default()).await?;
    //         for item in list.items {
    //             let name = item.name_any();
    //             let ns = item.metadata.namespace.map(|s| s + "/").unwrap_or_default();
    //             println!("\t\t{}{}", ns, name);
    //         }
    //     }
    // }

    Ok(())
}
