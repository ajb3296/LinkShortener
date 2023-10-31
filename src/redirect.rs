use crate::modules::database::get_url_from_id;
use crate::modules::convert::base62_to_base10;

use actix_web::{get, web, Responder, web::Redirect};

#[get("/s/{num}")]
async fn redirect(num: web::Path<String>, data: web::Data<mysql::Pool>) -> impl Responder {
    let num = num.into_inner();
    let num: u128 = base62_to_base10(num);
    let url = get_url_from_id(&data, num);

    Redirect::to(url).permanent()
}