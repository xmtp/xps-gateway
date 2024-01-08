use anyhow::Result;
use gateway::run;

#[tokio::main]
async fn main() -> Result<()> {
    crate::run().await?;
    Ok(())
}
