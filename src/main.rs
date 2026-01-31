mod todo_app;
mod routes;
use env_logger::Env;

use actix_web::{App, HttpServer, web,http::header};
use actix_cors::Cors;
use std::sync::Mutex;
use todo_app::TodoApp;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app = web::Data::new(Mutex::new(TodoApp::new()));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin().allow_any_method()
            .allow_any_header().max_age(3600);
        App::new()
            .wrap(cors)
            .app_data(app.clone())
            .route("/users/{user}/tasks", web::get().to(routes::get_tasks))
            .route("/users/{user}/tasks",web::post().to(routes::post_tasks))
            .route("/users/{user}/tasks/{id}",web::delete().to(routes::delete_task))

    })
        .bind(("127.0.0.1", 8080))?
        //Mon programme écoute sur le port 8080 et attend que quelqu’un lui parle en HTTP
        .run()
        .await
}
