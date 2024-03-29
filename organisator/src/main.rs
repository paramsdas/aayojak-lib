use std::io;

use life_track_hub::structures::todo::Todo;

fn main() {
    let mut todo_list: Vec<Todo> = Vec::new();
    loop {
        print!("1) Add ToDo\n2) View Exisiting Todos\n\nChoice:\n");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        if choice.trim() == "1" {
            let mut title = String::new();
            print!("\nTitle:\n");
            io::stdin()
                .read_line(&mut title)
                .expect("Failed to read line");
            let new_todo = Todo::new(&(title.trim())[..]);
            match new_todo {
                Ok(todo) => todo_list.push(todo),
                Err(err) => println!("Could not create ToDo: {}", err),
            }
        } else if choice.trim() == "2" {
            for (i, t) in todo_list.iter().enumerate() {
                println!("{{Id:{}, Title:{}}}", i, t.title());
            }
        } else {
            println!("Please enter a valid choice")
        }
    }
}
