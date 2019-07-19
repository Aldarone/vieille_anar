extern crate directories;
extern crate tini;
use directories::ProjectDirs;
use std::env;
use std::fs::create_dir_all;
use std::fs::File;
use std::path::Path;
use std::process;
use tini::Ini;

fn main() {
    //    let config = get_config().unwrap_or_else(|error| {
    //        println!("{}", error);
    //        process::exit(1);
    //    });
    //
    //    println!(
    //        "Instance: {}, api_key: {}, api_secret: {}",
    //        config.instance, config.api_key, config.api_secret
    //    );

    let args: Vec<String> = env::args().collect();
    let formatter = Formatter::from_args(&args).unwrap_or_else(|error| {
        println!("{}", error);
        process::exit(1);
    });

    println!(
        "Tu penses quoi {} ?\nJ'en ai rien à secouer, j'suis anarchiste et je voudrais foutre le feu {}",
        formatter.part_one, formatter.part_two
    );
}

struct Formatter {
    part_one: String,
    part_two: String,
}

impl Formatter {
    fn from_args(args: &[String]) -> Result<Formatter, &'static str> {
        if args.len() != 3 {
            return Err("You can only use two arguments");
        }

        let part_one = args[1].clone();
        let part_two = args[2].clone();

        return Ok(Formatter { part_one, part_two });
    }
}

struct Config {
    instance: String,
    api_key: String,
    api_secret: String,
}

impl Config {
    fn new(instance: String, api_key: String, api_secret: String) -> Result<Config, String> {
        if instance.is_empty() || api_key.is_empty() || api_secret.is_empty() {
            return Err(format!(
                    "Critial values are missing from the config file.\ninstance: {},\napi_key: {},\napi_secret: {}",
                    instance, api_key, api_secret
            ));
        }

        return Ok(Config {
            instance,
            api_key,
            api_secret,
        });
    }
}

fn get_config() -> Result<Config, String> {
    const SECTION_NAME: &str = "vieille_anar";

    match ProjectDirs::from("", "", "Vieille Anar") {
        Some(project_directory) => {
            let config_file = project_directory.config_dir().join("config.ini");
            let config_file = config_file.as_path();

            if !config_file.exists() {
                create_config_file(config_file);
            }

            let ini_config = Ini::from_file(config_file).unwrap();

            Config::new(
                ini_config
                    .get(SECTION_NAME, "instance")
                    .unwrap_or_else(|| String::new()),
                ini_config
                    .get(SECTION_NAME, "api_key")
                    .unwrap_or_else(|| String::new()),
                ini_config
                    .get(SECTION_NAME, "api_secret")
                    .unwrap_or_else(|| String::new()),
            )
        }
        None => panic!("Cannot find the project directories"),
    }
}

fn create_config_file(config_file: &Path) {
    if config_file.exists() {
        return;
    }

    let config_dir: &Path = config_file.parent().unwrap();

    let dir_error: String = format!("Can't create directory {}", config_dir.to_str().unwrap());
    create_dir_all(config_dir).expect(&dir_error);

    let file_error: String = format!("Can't create file {}", config_file.to_str().unwrap());
    File::create(config_file).expect(&file_error);
}
