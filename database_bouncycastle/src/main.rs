
mod bouncycastle;
use bouncycastle::poplate_db;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let iterations: i64 = 0;

    let _res = poplate_db().await?;

    Ok(())

}