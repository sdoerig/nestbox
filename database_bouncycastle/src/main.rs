
mod bouncycastle;
use bouncycastle::poplate_db;


fn main() -> mongodb::error::Result<()> {
    let _iterations: i64 = 0;

    let _res = poplate_db();

    Ok(())

}