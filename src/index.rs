use actix_web::{get, HttpResponse, Responder};

#[get("/")]
async fn index() -> impl Responder {
    let html = include_str!("../templates/index.html");
    HttpResponse::Ok().content_type("text/html").body(html)
}