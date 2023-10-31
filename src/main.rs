use std::env;
use actix_web::{web, App, HttpServer};
use actix_files as fs;

mod modules;
mod redirect;
mod index;
mod enroll;
mod interface;


fn get_conn_builder(
    db_user: String,
    db_password: String,
    db_host: String,
    db_port: u16,
    db_name: String,
) -> mysql::OptsBuilder {
    mysql::OptsBuilder::new()
        .ip_or_hostname(Some(db_host))
        .tcp_port(db_port)
        .db_name(Some(db_name))
        .user(Some(db_user))
        .pass(Some(db_password))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // initialize environment
    dotenvy::dotenv().ok();

    // initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    let db_user = env::var("MYSQL_USER").expect("MYSQL_USER is not set in .env file");
    let db_password = env::var("MYSQL_PASSWORD").expect("MYSQL_PASSWORD is not set in .env file");
    let db_host = env::var("MYSQL_HOST").expect("MYSQL_HOST is not set in .env file");
    let db_port = env::var("MYSQL_PORT").expect("MYSQL_PORT is not set in .env file");
    let db_name = env::var("MYSQL_DBNAME").expect("MYSQL_DBNAME is not set in .env file");
    let db_port = db_port.parse().unwrap();
    
    let builder = get_conn_builder(db_user, db_password, db_host, db_port, db_name);

    let pool = mysql::Pool::new(builder).unwrap();

    let shared_data = web::Data::new(pool);

    HttpServer::new(move || {
        App::new()
            .app_data(shared_data.clone())
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .service(redirect::redirect)
            .service(index::index)
            .service(enroll::enroll)
    })
    .bind(("0.0.0.0", 8080))?
    .workers(4)
    .run()
    .await
}