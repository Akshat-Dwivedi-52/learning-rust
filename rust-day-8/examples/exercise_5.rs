use serde::Serialize;
use anyhow::Result;

#[derive(Serialize)]
struct User {
    name: String,
    age: u32,
}

fn main() -> Result<()>{
    let database = vec![
        User {
            name: "Akshat Dwivedi".to_string(),
            age: 19,
        },
        User {
            name: "Karan Singh".to_string(),
            age: 18,
        },
    ];
    
    let json = serde_json::to_string_pretty(&database)?;
    println!("{}", json);
    
    Ok(())
}