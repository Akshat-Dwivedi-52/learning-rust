use rand::{RngExt, distr::Alphanumeric};
use std::io::{self, Write};

#[derive(Debug, Clone)]
struct Todo {
    id: String,
    title: String,
    completed: bool,
}

#[derive(Debug)]
enum Command {
    Add,
    Mark,
    Remove,
    List,
    Exit,
}

// Funtions ------------------------------------------

fn get_user_command() -> String {
    let mut input = String::new();
    print!("Enter command: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}

fn generate_random_id() -> String {
    let id: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(4)
        .map(char::from)
        .collect();

    id
}

fn add_todo() -> Todo {
    let mut title = String::new();
    print!("Give Title of Todo: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read input");

    let title = title.trim().to_string();
    let new_todo = Todo {
        id: generate_random_id(),
        title: title,
        completed: false,
    };

    println!("Task Added Succesfully!!😆");
    new_todo
}

fn mark_todo(todo_list: &mut Vec<Todo>) {
    let mut todo_id = String::new();
    print!("Give ID of Todo: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut todo_id)
        .expect("Failed to read input");
    let todo_id = todo_id.trim();
    if !todo_id.is_empty() {
        for list in todo_list.iter_mut() {
            if list.id == todo_id {
                list.completed = true;
                println!("Updated Successfully!!");
                return;
            }
        }
    }
}

fn remove_todo(todo_list: &mut Vec<Todo>) {
    let mut todo_id = String::new();
    print!("Give ID of Todo to Remove: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut todo_id)
        .expect("Failed to read input");
    let todo_id = todo_id.trim();

    let mut found_index = None;

    if !todo_id.is_empty() {
        for (index, todo) in todo_list.iter().enumerate() {
            // println!("{}, {:?}", index, todo);
            if todo.id == todo_id {
                found_index = Some(index);
                break;
            }
        }

        match found_index {
            Some(index) => {
                todo_list.remove(index);
                println!("Todo Removed!!")
            }
            None => {
                println!("not found.");
            }
        }
    }
}

// Main Application Code -----------------------------

fn main() {
    println!("--------------------------------");
    println!("----  CLI Todo Application  ----");
    println!("--------------------------------\n");

    let mut todo_list: Vec<Todo> = Vec::new();

    loop {
        let input_str = get_user_command();

        let command = match input_str.as_str() {
            "Add" => Command::Add,
            "Mark" => Command::Mark,
            "Remove" => Command::Remove,
            "List" => Command::List,
            "Exit" => Command::Exit,
            _ => {
                continue;
            }
        };

        // 4. Route the application control flow
        match command {
            Command::Exit => {
                println!("Exiting application. Goodbye!");
                break;
            }
            Command::Add => {
                println!("\n--------------------------------\n");
                let result = add_todo();
                todo_list.push(result);
                println!("\n--------------------------------\n");
            }
            Command::List => {
                println!("\n--------------------------------\n");

                for list in &todo_list {
                    print!("ID: {:?}", list.id);
                    print!(", TITLE: {:?}", list.title);
                    print!(", STATUS: {:?}", list.completed);
                    println!("");
                }

                println!("\n--------------------------------\n");
            }
            Command::Mark => {
                println!("\n--------------------------------\n");

                mark_todo(&mut todo_list);

                println!("\n--------------------------------\n");
            }
            Command::Remove => {
                println!("\n--------------------------------\n");

                remove_todo(&mut todo_list);

                println!("\n--------------------------------\n");
            }
        }
    }
}
