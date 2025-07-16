use std::io;

fn main() {
    let mut todos: Vec<String> = vec![];

    loop {
        println!("\n1. Add a todo");
        println!("2. View todos");
        println!("3. Remove a todo");
        println!("4. Exit\n");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\nInvalid choice, please enter a number between 1 and 4.");
                continue;
            }
        };

        match input {
            1 => {
                println!("Enter a todo:");
                let mut todo = String::new();
                io::stdin()
                    .read_line(&mut todo)
                    .expect("Failed to read line");
                todos.push(todo.trim().to_string());
            }
            2 => {
                println!("Todos:");
                for todo in &todos {
                    println!("- {}", todo);
                }
            }
            3 => {
                println!("Enter the index of the todo to remove:");
                let mut index = String::new();
                io::stdin()
                    .read_line(&mut index)
                    .expect("Failed to read line");
                let index: usize = match index.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("\nInvalid index, please enter a number.");
                        continue;
                    }
                };
                if index < todos.len() {
                    todos.remove(index);
                } else {
                    println!("\nInvalid index, please enter a valid index.");
                }
            }
            4 => {
                break;
            }
            _ => {
                println!("Invalid choice, please enter a number between 1 and 4.");
            }
        }
    }
}
