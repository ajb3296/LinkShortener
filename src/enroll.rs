use crate::interface::Url;

use crate::modules::database::enroll_to_db;
use crate::modules::convert::base10_to_base62;

use actix_web::{post, web, HttpResponse};

#[post("/enroll/")]
async fn enroll(post: web::Json<Url>, data: web::Data<mysql::Pool>) -> HttpResponse {
    let url = post.url.clone();
    let id = enroll_to_db(&data, url).unwrap();
    let id = base10_to_base62(id);

    HttpResponse::Ok().json(Url { url: id })
}