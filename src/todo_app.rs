use serde::{Serialize, Deserialize};
use std::fs;
use chrono::NaiveTime;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub(crate) struct Task {
    id: usize,
    title: String,
    done: bool,
    start: Option<NaiveTime>,
    end: Option<NaiveTime>
}

#[derive(Serialize, Deserialize)]
pub(crate) struct TodoList {
    pub(crate) tasks: Vec<Task>,
}
#[derive(Serialize, Deserialize)]
pub(crate) struct TodoApp {
    pub(crate) users: HashMap<String, TodoList>,
}
impl Task {
    fn print_task(&self) {
        let fait = if self.done { "X" } else { " " };
        println!("[{}] {} - {} ({:?} -> {:?})", fait, self.id, self.title, self.start, self.end);
    }
}
impl TodoList {
    fn new() -> Self {
        TodoList{
            tasks : Vec::new()
        }
    }
    fn add(&mut self, title: String, debut : Option<NaiveTime>,fin : Option<NaiveTime>) {
        if let (Some(start), Some(end)) = (debut, fin) {
            if self.overlaps(start, end) {
                println!("Impossible d'ajouter : chevauchement détecté !");
                return;
            }
        }
        let new_task =
            Task {
                id : self.tasks.len(),
                title,
                done : false,
                start : debut,
                end : fin,
            };
        self.tasks.push(new_task);
    }
    // Exemple : [X] 1 - Réviser Rust (09:00 -> 10:00)
    fn list(&self) {
        for task in self.tasks.iter() {
            task.print_task();
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
    fn list_done(&self){
        for task in self.tasks.iter().filter( | x| x.done) {
            task.print_task();
        }
    }
    fn overlaps(&self, new_start: NaiveTime, new_end: NaiveTime) -> bool { //True si la nouvelle tâche chevauche une autre
        for task in self.tasks.iter() {
            if let (Some(start),Some(end))= (task.start,task.end){
                if start < new_end && new_start < end {
                    return true
                }
            }
        }
        false
    }
}

impl TodoApp {
    pub(crate) fn new() -> Self {
        TodoApp{
            users :HashMap::new()
        }
    }
    fn get_or_create_user(&mut self, username: &str) -> &mut TodoList{
        self.users.entry(username.to_string()).or_insert_with(|| TodoList::new())
    }
    pub(crate) fn add_task(&mut self, username: &str, title: String, start: Option<NaiveTime>, end: Option<NaiveTime>){
        let list = self.get_or_create_user(username);
        list.add(title,start,end);
    }
    fn list_tasks(&self, username: &str){
        let liste = self.users.get(username);
        if let Some(l) = liste {
            l.list()
        }
    }
    fn save(&self, filename: &str) -> Result<(), std::io::Error>{
        let contenu = serde_json::to_string_pretty(&self)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        fs::write(filename, contenu)?;
        Ok(())
    }
    fn load(filename: &str) -> Result<Self, std::io::Error> {
        let contenu = fs::read_to_string(filename)?;
        let data = serde_json::from_str(&contenu)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        Ok(data)
    }
    fn mark_done(&mut self, username: &str, id: usize) {
        if let Some(list) = self.users.get_mut(username) {
            list.mark_done(id);
        }
    }
    fn remove_task(&mut self, username: &str, id: usize){
        if let Some(list) = self.users.get_mut(username) {
            list.remove(id);
        }
    }
}
#[derive(Serialize, Deserialize)]
pub struct TaskInput{
    pub title: String,
    pub start: Option<NaiveTime>,
    pub end: Option<NaiveTime>,
}

fn main() {
    let mut todo_list = TodoList::new();
    todo_list.list();
    let start = NaiveTime::from_hms_opt(9, 0, 0);
    let end = NaiveTime::from_hms_opt(10, 30, 0);
    todo_list.add(String::from("Réviser Rust"),start,end);
    todo_list.mark_done(0);
    todo_list.save("todo_list.json");
    todo_list.remove(0);
    let mut app = TodoApp::new();
    let tache = "Réviser maths".to_string();
    let tache2 = "Faire du sport".to_string();
    app.add_task("Toto",tache,NaiveTime::from_hms_opt(15,0,0),NaiveTime::from_hms_opt(15,45,0));
    app.add_task("Toto",tache2,NaiveTime::from_hms_opt(14,0,0),NaiveTime::from_hms_opt(15,45,0));
    app.mark_done("Toto",0);
    app.list_tasks("Toto");
}