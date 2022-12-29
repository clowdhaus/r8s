mod versions;

use kube::Client;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let deprecated_versions = versions::Deprecated::get()?;
    println!("{deprecated_versions:#?}");

    let client = Client::try_default().await?;
    let current_versions = versions::Cluster::get(&client).await?;
    println!("{current_versions:#?}");

    Ok(())
}
