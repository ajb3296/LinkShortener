use std::env;
use std::path::Path;
use actix_files as fs;
use actix_web::{web, App, HttpServer};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

mod modules;
mod redirect;
mod index;
mod enroll;
mod structs;


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

    let file_path = "key.pem";
    
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    
    let path = Path::new(file_path);
    if path.exists() {
        println!("Starting server with SSL");
        builder.set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
        builder.set_certificate_chain_file("cert.pem").unwrap();
        
        HttpServer::new(move || {
            App::new()
                .app_data(shared_data.clone())
                .service(fs::Files::new("/static", "./static").show_files_listing())
                .service(index::index)
                .service(redirect::redirect)
                .service(enroll::enroll)
        })
        .bind_openssl("0.0.0.0:8080", builder)?
        .workers(4)
        .run()
        .await
    } else {
        println!("Starting server without SSL");
        HttpServer::new(move || {
            App::new()
                .app_data(shared_data.clone())
                .service(fs::Files::new("/static", "./static").show_files_listing())
                .service(index::index)
                .service(redirect::redirect)
                .service(enroll::enroll)
        })
        .bind(("0.0.0.0", 8000))?
        .workers(4)
        .run()
        .await
    }
}
