use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

#[derive(Debug)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

struct TodoList {
    tasks: HashMap<usize, Task>,
    next_id: usize,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList {
            tasks: HashMap::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, description: String) {
        let task = Task {
            id: self.next_id,
            description,
            completed: false,
        };
        self.tasks.insert(self.next_id, task);
        self.next_id += 1;
    }

    fn complete_task(&mut self, id: usize) -> Result<(), String> {
        match self.tasks.get_mut(&id) {
            Some(task) => {
                task.completed = true;
                Ok(())
            }
            None => Err(format!("Task with id {} not found", id)),
        }
    }

    fn list_tasks(&self) {
        for task in self.tasks.values() {
            println!(
                "{}: {} [{}]",
                task.id,
                task.description,
                if task.completed { "✓" } else { " " }
            );
        }
    }

    fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(filename)?;

        for task in self.tasks.values() {
            writeln!(
                file,
                "{},{},{}",
                task.id, task.description, task.completed
            )?;
        }

        Ok(())
    }

    fn load_from_file(&mut self, filename: &str) -> io::Result<()> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        self.tasks.clear();
        self.next_id = 1;

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 3 {
                let id: usize = parts[0].parse().unwrap_or(0);
                let description = parts[1].to_string();
                let completed: bool = parts[2].parse().unwrap_or(false);

                let task = Task {
                    id,
                    description,
                    completed,
                };

                self.tasks.insert(id, task);
                self.next_id = self.next_id.max(id + 1);
            }
        }

        Ok(())
    }
}

fn main() -> io::Result<()> {
    let mut todo_list = TodoList::new();
    let filename = "todo_list.txt";

    if Path::new(filename).exists() {
        todo_list.load_from_file(filename)?;
        println!("Loaded existing todo list.");
    }

    loop {
        println!("\nTodo List Manager");
        println!("1. Add task");
        println!("2. Complete task");
        println!("3. List tasks");
        println!("4. Save and quit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

        match choice.trim() {
            "1" => {
                println!("Enter task description:");
                let mut description = String::new();
                io::stdin().read_line(&mut description)?;
                todo_list.add_task(description.trim().to_string());
                println!("Task added.");
            }
            "2" => {
                println!("Enter task ID to complete:");
                let mut id_str = String::new();
                io::stdin().read_line(&mut id_str)?;
                if let Ok(id) = id_str.trim().parse() {
                    match todo_list.complete_task(id) {
                        Ok(_) => println!("Task completed."),
                        Err(e) => println!("{}", e),
                    }
                } else {
                    println!("Invalid task ID.");
                }
            }
            "3" => {
                todo_list.list_tasks();
            }
            "4" => {
                todo_list.save_to_file(filename)?;
                println!("Todo list saved. Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }

    Ok(())
}
