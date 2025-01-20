pub mod config;
pub mod test;
pub mod generators;
pub mod utils;
pub mod read;
pub mod write;
pub mod error;

fn main() {
    match config::Config::read_config("../cheetah.config.json") {
        Ok(config) => {
            println!("Config loaded successfully {:#?}", config);
        }
        Err(e) => {
            eprintln!("Error reading config: {}", e);
        }
    }
}