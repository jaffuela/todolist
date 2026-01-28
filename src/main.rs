mod todo_app;
mod routes;

use actix_web::{App, HttpServer, web};
use todo_app::TodoApp;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app = web::Data::new(TodoApp::new());

    HttpServer::new(move || {
        App::new()
            .app_data(app.clone())
            .route("/tasks/{user}", web::get().to(routes::get_tasks))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
