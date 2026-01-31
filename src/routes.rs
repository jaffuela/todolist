use std::sync::Mutex;
use actix_web::{web, HttpResponse, Responder};
use crate::todo_app::{TodoApp, Task, TaskInput};
use log::{info, error,debug};

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
        HttpResponse::Ok().json(Vec::<Task>::new())
    }

}
pub async fn post_tasks(
    data: web::Data<Mutex<TodoApp>>,
    username: web::Path<String>,
    task: web::Json<TaskInput>,
) -> impl Responder {
    info!("Appel POST pour l'utilisateur {:?}",username);
    let mut app = data.lock().unwrap();
    let user = username.into_inner();
    let input = task.into_inner();
    info!("Requête reçue - User: {}, Title: {}", user, input.title);
    debug!("Détails du temps - Start: {:?}, End: {:?}", input.start, input.end);
    app.add_task(&user,input.title,input.start,input.end);
    info!("Tâche ajoutée avec succès");
    HttpResponse::Created().finish()
}

pub async fn delete_task(
    data: web::Data<Mutex<TodoApp>>,
    path: web::Path<(String, usize)>,
) -> impl Responder{
    let mut app = data.lock().unwrap();
    let (user, task_id) = path.into_inner();
    app.remove_task(&user,task_id);
    HttpResponse::NoContent().finish()
}
