use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize)]
struct Task {
    id: usize,
    title: String,
    done: bool,
}

#[derive(Serialize, Deserialize)]
struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    fn new() -> Self {
        TodoList{
            tasks : Vec::new()
        }
    }
}

impl TodoList {
    fn add(&mut self, title: String) {
        let new_task =
            Task {
                id : self.tasks.len()+1,
                title : title,
                done : false
            };
        self.tasks.push(new_task);
    }
}
//Exemple
//[ ] 1 - Réviser Rust
//[X] 2 - Faire le projet

impl TodoList {
    fn list(&self) {
        for task in self.tasks.iter() {
            let fait = if task.done { "X" } else { " " };
            println!("[{}] {} - {}", fait, task.id, task.title);
        }
    }
    fn mark_done(&mut self, id: usize) {
        for task in self.tasks.iter_mut() {
            if task.id == id {
                task.done = true;
                break;
            }
        }
    }
    fn remove(&mut self, id: usize) {
        self.tasks.retain(|x| x.id != id);
    }
    fn save(&self, filename: &str) -> Result<(), std::io::Error> {
        let contenu = serde_json::to_string_pretty(&self)?;
        fs::write(filename, contenu)?;
        Ok(())
    }
    fn load(filename: &str) -> Result<Self, std::io::Error>{
        let contenu = fs::read_to_string(filename)?;
        serde_json::from_str(&contenu)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }
}

fn main() {
    let mut todo_list = TodoList::new();
    todo_list.list();
    todo_list.add(String::from("Hello, world!"));
    todo_list.mark_done(0);

}