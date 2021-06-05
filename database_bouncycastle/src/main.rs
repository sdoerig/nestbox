use std::process;
mod bouncycastle;
use bouncycastle::populate_db;
mod extract_argv;

use extract_argv::extract_argv;

fn main() {
    let (db_uri, db_name, record_int) = extract_argv();

    let _res = populate_db(&db_uri, &db_name, record_int);

    process::exit(0)
}
