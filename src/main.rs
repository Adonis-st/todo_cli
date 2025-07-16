use std::io;

struct Todo {
    title: String,
    completed: bool,
}

fn main() {
    let mut todos: Vec<Todo> = vec![];

    loop {
        println!("\n1. Add a todo");
        println!("2. View todos");
        println!("3. Toggle a todo");
        println!("4. Remove a todo");
        println!("5. Exit\n");

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
                todos.push(Todo {
                    title: todo.trim().to_string(),
                    completed: false,
                });
            }
            2 => {
                println!("Todos:");

                for todo in &todos {
                    let status = if todo.completed { "✅" } else { "[]" };
                    println!("- {} {}", status, todo.title);
                }
            }
            3 => {
                println!("Enter the index of the todo to toggle:");
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
                    todos[index].completed = !todos[index].completed;
                } else {
                    println!("\nInvalid index, please enter a valid index.");
                }
            }
            4 => {
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
            5 => {
                break;
            }
            _ => {
                println!("Invalid choice, please enter a number between 1 and 4.");
            }
        }
    }
}
