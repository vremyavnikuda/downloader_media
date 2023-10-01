use std::fs;
use actix_web::{HttpResponse, Error as ActixError};


pub(crate) async fn index() -> Result<HttpResponse, ActixError> {
    let content = match fs::read_to_string("src/index.html") {
        Ok(content) => {
            log::info!("Успешный переход по /index");
            content
        }
        Err(_) => {
            log::error!("Не удалось загрузить index.html");
            return Err(actix_web::error::ErrorInternalServerError("index.html not found"));
        }
    };

    Ok(HttpResponse::Ok().content_type("text/html").body(content))
}
