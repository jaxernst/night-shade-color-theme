use std::collections::HashMap;
use std::io;

enum Command {
    Add { title: String, author: String },
    Remove(String),
    Get(String),
    List,
    Quit,
}

impl Command {
    fn from_input(input: &str) -> Option<Command> {
        let parts: Vec<&str> = input.trim().splitn(3, ' ').collect();
        match parts[0] {
            "add" => Some(Command::Add {
                title: parts[1].to_string(),
                author: parts[2].to_string(),
            }),
            "remove" => Some(Command::Remove(parts[1].to_string())),
            "get" => Some(Command::Get(parts[1].to_string())),
            "list" => Some(Command::List),
            "quit" => Some(Command::Quit),
            _ => None,
        }
    }
}

struct Library {
    books: HashMap<String, String>,
}

impl Library {
    fn new() -> Library {
        Library { books: HashMap::new() }
    }

    fn add_book(&mut self, title: String, author: String) {
        self.books.insert(title, author);
    }

    fn remove_book(&mut self, title: &str) -> Result<(), String> {
        self.books.remove(title).map_or_else(
            || Err("Book not found.".to_string()),
            |_| Ok(()),
        )
    }

    fn get_book(&self, title: &str) -> Option<&String> {
        self.books.get(title)
    }

    fn list_books(&self) {
        for (title, author) in &self.books {
            println!("{} by {}", title, author);
        }
    }
}

fn main() {
    let mut library = Library::new();
    loop {
        println!("Enter command:");
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input");
            continue;
        }

        match Command::from_input(&input) {
            Some(Command::Add { title, author }) => {
                library.add_book(title, author);
                println!("Book added.");
            },
            Some(Command::Remove(title)) => match library.remove_book(&title) {
                Ok(_) => println!("Book removed."),
                Err(e) => println!("{}", e),
            },
            Some(Command::Get(title)) => {
                if let Some(author) = library.get_book(&title) {
                    println!("{} by {}", title, author);
                } else {
                    println!("Book not found.");
                }
            },
            Some(Command::List) => {
                library.list_books();
            },
            Some(Command::Quit) => {
                println!("Exiting...");
                break;
            },
            None => println!("Invalid command"),
        }
    }
}
