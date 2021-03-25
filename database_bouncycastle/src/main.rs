
use std::process;
mod bouncycastle;
use bouncycastle::poplate_db;
mod extract_argv;
use extract_argv::extract_argv;

fn main() -> () {
    let (db_uri, record_int) = extract_argv();

    let _res = poplate_db(&db_uri, record_int);

    process::exit(0)

}

