use std::sync::Mutex;
use actix_web::{web, HttpResponse, Responder};
use crate::todo_app::{Task, TodoApp, TaskInput};

pub async fn get_tasks(
    data: web::Data<Mutex<TodoApp>>,
    username: web::Path<String>,
) -> impl Responder {
    let app = data.lock().unwrap();
    let user = username.into_inner();
    if let Some(list)=app.users.get(&user){
        HttpResponse::Ok().json(&list.tasks)
    }
    else{
        HttpResponse::Ok().json(Vec::<()>::new())
    }

}
pub async fn post_tasks(
    data: web::Data<Mutex<TodoApp>>,
    username: web::Path<String>,
    task: web::Json<TaskInput>,
) -> impl Responder {
    let mut app = data.lock().unwrap();
    let user = username.into_inner();
    let input = task.into_inner();
    app.add_task(&user,input.title,input.start,input.end);
    HttpResponse::Created().finish()
}