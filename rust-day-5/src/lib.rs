
#[derive(Debug)]
pub struct Config<'a>{
    pub host: &'a str,
    pub port: &'a str,
    pub database: &'a str,
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(Debug)]
pub enum ParseError<'a> {
    UnknownKey(&'a str),
    MissingField(&'static str),
    InvalidFormat,
}

pub fn parse_config<'a>(input: &'a str) -> Result<Config<'a>, ParseError<'a>> {
    
    let mut host = "";
    let mut port = "";
    let mut database = "";
    let mut username = "";
    let mut password = "";

    for line in input.lines() {
        // let (key, value) = line.split_once("=").unwrap();

        let Some((key, value)) = line.split_once('=') else {
            return Err(ParseError::InvalidFormat);
        };

        match key {
            "host" => host = value,
            "port" => port = value,
            "database" => database = value,
            "username" => username = value,
            "password" => password = value,
            _ => return Err(ParseError::UnknownKey(key)),
        }
    }

    if host.is_empty() {
        return Err(ParseError::MissingField("host"));
    } else if port.is_empty() {
        return Err(ParseError::MissingField("port"));
    } else if database.is_empty() {
        return Err(ParseError::MissingField("database"));
    } else if username.is_empty() {
        return Err(ParseError::MissingField("username"));
    } else if password.is_empty() {
        return Err(ParseError::MissingField("password"));
    }


    Ok(Config { host, port, database, username, password })
}