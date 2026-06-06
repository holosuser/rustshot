use sheller::run;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Config {
    path: String
}

fn createconfigfile() {
    let home = std::env::var("HOME").unwrap();
    let cd = format!("{}/.config/rustshot", home);
    let we = format!("{}/.config/rustshot/config.toml", home);
    fs::create_dir(&cd).unwrap();
    fs::write(&we, "path = \"\"").unwrap();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|arg: &String|arg=="--help"){
        println!("rustshot:
    --create-config = this argument create config dir in your home directory, and config TOML file.
    shot = make screenshot.")
    }
    if args.iter().any(|arg: &String|arg=="--create-config"){
       createconfigfile();
       return;
    }
    let home = std::env::var("HOME").unwrap();
    let config_path = format!("{}/.config/rustshot/config.toml", home);
    let con = fs::read_to_string(&config_path).unwrap_or_else(|_| {
        eprintln!("rustshot: config not found. Run with --help first.");
        std::process::exit(1);
    });
    let cfg: Config = toml::from_str(&con).unwrap();
    if args.iter().any(|arg: &String|arg=="shot"){
        let cmd = format!(r#"grim -g "$(slurp)" - | tee {}$(date +'%Y-%m-%d_%H-%M-%S').png | wl-copy"#, cfg.path);
        run!("{}", cmd);
    }
}
