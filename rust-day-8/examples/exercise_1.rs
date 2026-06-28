fn main() {
    match divide(20, 0) {
        Ok(value) => println!("{}", value),
        Err(err) => println!("{}", err),
    }
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err(String::from("B Can't be 0."));
    }
    
    let result: i32 = a / b;
    Ok(result)
}