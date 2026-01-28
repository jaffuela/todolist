use actix_web::{web, HttpResponse, Responder};
use crate::todo_app::TodoApp;

pub async fn get_tasks(
    data: web::Data<TodoApp>,
    username: web::Path<String>,
) -> impl Responder {
    let app = data.get_ref();
    let user = username.into_inner();
    if let Some(list)=app.users.get(&user){
        HttpResponse::Ok().json(&list.tasks)
    }
    else{
        HttpResponse::Ok().json(Vec::<()>::new())
    }

}