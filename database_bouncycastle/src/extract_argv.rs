use getopts::Options;
use std::env;
use std::process;

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} -d mongodb://127.0.2.15:27017/?w=majority -n 123", program);
    print!("{}", opts.usage(&brief));
}



pub fn extract_argv() -> (String, i32) {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optopt("d", "database_uri", 
    "URI to mongodb e.g mongodb://<db_host>:<db_port>/",
    "MONGO_DB_URI");
    opts.optopt("n", "number of records to insert", 
    "The number of records to insert",
    "NUMBER");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(_f) => {
            print_usage(&program, &opts);
            process::exit(2)
        }
    };
    let db_uri = match matches.opt_str("d") {
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
    (db_uri, record_int)
}