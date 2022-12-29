mod cluster;
mod deprecated;

use kube::Client;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let deprecated = deprecated::Deprecated::get()?;
    // println!("{deprecated:#?}");

    let client = Client::try_default().await?;
    let discovery = cluster::Discovery::get(&client).await?;
    // println!("{discovery:#?}");

    for (key, value) in &deprecated.versions {
        if discovery.versions.contains_key(key) {
            println!("{key} is deprecated");
            println!("{value:#?}");
        }
    }

    Ok(())
}
