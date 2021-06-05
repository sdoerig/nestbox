use getopts::Options;
use std::env;
use std::process;

fn print_usage(program: &str, opts: &Options) {
    let brief = format!(
        "Usage: {} -m mongodb://127.0.0.1:27017/?w=majority -d nestbox_bouncycastle -n 123",
        program
    );
    print!("{}", opts.usage(&brief));
}

pub fn extract_argv() -> (String, String, i32) {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optopt(
        "m",
        "mongodb_host",
        "URI to mongodb e.g mongodb://<db_host>:<db_port>/",
        "MONGO_DB_HOST",
    );
    opts.optopt(
        "d",
        "database_name",
        "dateabase name e.g. nestbox_bouncycastle",
        "MONGO_DB_HOST",
    );
    opts.optopt(
        "n",
        "number of records to insert",
        "The number of records to insert",
        "NUMBER",
    );
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(_f) => {
            print_usage(&program, &opts);
            process::exit(2)
        }
    };
    let db_host = match matches.opt_str("m") {
        Some(m) => m,
        None => {
            print_usage(&program, &opts);
            process::exit(3)
        }
    };
    let db_name = match matches.opt_str("d") {
        Some(m) => m,
        None => {
            print_usage(&program, &opts);
            process::exit(3)
        }
    };
    let record_str = match matches.opt_str("n") {
        Some(m) => m,
        None => {
            print_usage(&program, &opts);
            process::exit(3)
        }
    };
    let record_int = match record_str.parse::<i32>() {
        Ok(i) => i,
        Err(_f) => {
            print_usage(&program, &opts);
            process::exit(3)
        }
    };
    (db_host, db_name, record_int)
}
