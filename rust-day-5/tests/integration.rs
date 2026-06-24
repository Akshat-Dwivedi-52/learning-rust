use rust_day_5::parse_config;

#[test]
fn valid_config() {
    
    let config_data = "\
host=localhost
port=5432
database=mydb
username=admin
password=secret";

    let result = parse_config(&config_data);

    assert!(result.is_ok());

    let config = result.unwrap();

    assert_eq!(config.host, "localhost");
    assert_eq!(config.port, "5432");
    assert_eq!(config.database, "mydb");
    assert_eq!(config.username, "admin");
    assert_eq!(config.password, "secret");
}

#[test]
fn unknown_key_returns_error() {
        let config_data = "\
host=localhost
port=5432
database=mydb
username=admin
password=secret
country=BHARAT"
;

    let result = parse_config(&config_data);

    assert!(result.is_err());
}

#[test]
fn missing_password_returns_error() {
    
    let config_data = "\
host=localhost
port=5432
database=mydb
username=admin";

    let result = parse_config(&config_data);

    assert!(result.is_err());
}

#[test]
fn invalid_format_returns_error() {

    let config_data = "\
host=localhost
port=5432
database=mydb
username=admin
password_secret"; // Error Planted here : `_` instead of =

    let result = parse_config(&config_data);
    assert!(result.is_err());
}