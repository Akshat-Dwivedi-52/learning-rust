use anyhow::Result;

#[derive(Debug)]
enum AppError {
    _Kaboom,
    _SixSeven,
    UserNotFound,
    _DatabaseEmpty,
}

fn main() {
    let mut users: Vec<&str> = Vec::new();
    
    users.push("Akshat Dwivedi");
    users.push("Nitin Chauhan");
    users.push("Karan Singh");
    users.push("Bhavya");
    
    match find_user(users) {
        Ok(value) => println!("{}", value),
        Err(err) => println!("{:?}", err),
    }
}

fn find_user(database: Vec<&str>) -> Result<String, AppError> {
    let search_user = String::from("ChatGPT");

    for user in database {
        if user == search_user {
            return Ok(user.to_string());
        }
    }

    return Err(AppError::UserNotFound);
}