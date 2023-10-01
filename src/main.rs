mod manual_hello;
mod echo;
mod hello;
mod index;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use crate::manual_hello::manual_hello;
use crate::index::index;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //логирование в консоль
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .service(crate::hello::hello)
            .service(crate::echo::echo)

            .route("/hey", web::get().to(manual_hello))
            .route("/index",web::get().to(index))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}