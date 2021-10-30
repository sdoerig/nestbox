use getopts::Options;
use std::process;
use std::{env};
use yaml_rust::{YamlLoader};

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} -c CONFIG_FILE", program);
    print!("{}", opts.usage(&brief));
}

pub fn extract_argv() -> String {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optopt("c", "config", "Path to configuration file", "CONFIG_FILE");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(_f) => {
            print_usage(&program, &opts);
            process::exit(2)
        }
    };
    match matches.opt_str("c") {
        Some(m) => m,
        None => {
            print_usage(&program, &opts);
            process::exit(3)
        }
    }
}

pub struct Config {
    pub mongodb_uri: String,
    pub mongodb_database: String,
    pub httpserver_ip: String,
    pub httpserver_port: String,
    pub image_directory: String,
}

pub fn parse_yaml(config_file: String) -> Config {
    let config_str = match std::fs::read_to_string(&config_file) {
        Ok(s) => s,
        Err(_e) => process::exit(4),
    };
    let config_yaml = YamlLoader::load_from_str(&config_str).unwrap();
    let config_doc = &config_yaml[0];
    Config {
        mongodb_uri: config_doc["mongodb"]["uri"].as_str().unwrap().to_string(),
        mongodb_database: config_doc["mongodb"]["database"]
            .as_str()
            .unwrap()
            .to_string(),
        httpserver_ip: config_doc["httpserver"]["ip"].as_str().unwrap().to_string(),
        httpserver_port: config_doc["httpserver"]["port"].as_str().unwrap().to_string(),
        image_directory: config_doc["images"]["directory"].as_str().unwrap().to_string()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_config() {
        let config = parse_yaml(String::from("nestboxd_conf.yaml"));
        assert_eq!(config.httpserver_ip, String::from("127.0.0.1"));
        assert_eq!(config.image_directory, String::from("/home/doerig/temp/nestbox_images"))
    }
}