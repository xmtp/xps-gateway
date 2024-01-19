use anyhow::Result;
use xps_gateway::run;

#[tokio::main]
async fn main() -> Result<()> {
    crate::run().await?;
    Ok(())
}
