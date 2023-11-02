use crate::structs::{GetFullUrl, ShortUrlResponse};

use crate::modules::database::enroll_to_db;
use crate::modules::convert::base10_to_base62;

use actix_web::{post, web, HttpResponse};

use url::Url;

#[post("/enroll/")]
async fn enroll(post: web::Json<GetFullUrl>, data: web::Data<mysql::Pool>) -> HttpResponse {
    let mut url = post.url.clone();

    url = url.replace(" ", "").replace("\n", "");

    if url.is_empty() {
        return HttpResponse::BadRequest().body("Empty URL");
    }

    if Url::parse(&url).is_err() {
        url = format!("https://{}", url);
    }

    let id = match enroll_to_db(&data, url) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Database error"),
    };
    let id = base10_to_base62(id);

    HttpResponse::Ok().json(ShortUrlResponse {
                                url: id
                            })
}