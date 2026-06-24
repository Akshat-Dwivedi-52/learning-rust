use rust_day_5::parse_config;

fn main() {

    let config_data = std::fs::read_to_string("config.txt").expect("Failed to read config file!!");

    match parse_config(&config_data) {
        Ok(config) => println!("{:#?}", config),
        Err(err) => println!("{:?}", err),
    }

}


// Write tests for:
// Valid config ✅
// Unknown key ✅
// Missing field ✅
// Invalid format ✅