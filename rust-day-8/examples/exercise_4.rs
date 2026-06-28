use serde::Deserialize;
use anyhow::Result;

#[derive(Deserialize)]
struct User {
    name: String,
    age: u32,
}

fn main() -> Result<()> {
    let text = r#"{"name":"Akshat","age":19}"#;
    let details: User = serde_json::from_str(text)?;
    
    println!("{}", details.name);
    println!("{}", details.age);
    
    Ok(())
}