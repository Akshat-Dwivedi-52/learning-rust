use std::{
    collections::HashMap,
    io::{self, Write},
};

fn get_user_command() -> String {
    let mut input = String::new();
    print!("\nEnter command: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

struct Database {
    records: HashMap<String, String>,
}

impl Database {
    fn set(&mut self, command: String) {
        // Removing the `SET`.
        let filtered_cmd = command
            .split_whitespace()
            .skip(1)
            .collect::<Vec<_>>()
            .join(" ");

        // Filtering the KEY
        let key = filtered_cmd.split_whitespace().next().unwrap();

        // Filtering the VALUE.
        let value = filtered_cmd
            .split_whitespace()
            .skip(1)
            .collect::<Vec<_>>()
            .join(" ");

        // Inserting the DATA :
        self.records.insert(key.to_string(), value);
        println!("+OK");
    }

    fn get(&self, command: String) {
        // Extracting the KEY.
        let key = command
            .split_whitespace()
            .skip(1)
            .collect::<Vec<_>>()
            .join("");

        match self.records.get(&key) {
            Some(value) => {
                println!("> {}", value);
                println!("+OK");
            }
            None => println!("Key Not Found!"),
        }
    }

    fn delete(&mut self, command: String) {
        let key = command
            .split_whitespace()
            .skip(1)
            .collect::<Vec<_>>()
            .join("");

        match self.records.remove(&key) {
            Some(_) => println!("+OK"),
            None => println!("Key Not Found !"),
        }
    }

    fn list(&self) {
        for (key, value) in &self.records {
            println!("{} -> {}", key, value);
        }
    }
}

// Application
fn main() {
    let mut db = Database {
        records: HashMap::new(),
    };

    loop {
        let userinput = get_user_command();

        if userinput.starts_with("SET ") {
            db.set(userinput);
        } else if userinput.starts_with("GET ") {
            db.get(userinput);
        } else if userinput.starts_with("DELETE ") {
            db.delete(userinput);
        } else if userinput == "LIST" {
            db.list();
        } else if userinput == "EXIT" {
            println!("GoodBye!");
            break;
        } else {
            println!("> Try Again!");
        }
    }
}
