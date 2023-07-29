mod entities;
mod repositories;
mod controllers;
mod services;

use controllers::{users_controller, cities_controller};
use actix_web::{App, HttpServer};

fn main() {
    let addrs = "127.0.0.1";
    let port = 8080;
    println!("webserver is listening on {}:{}", &addrs, &port);

    #[actix_web::main]
    async fn webserver(addrs: &str, port: u16) -> std::io::Result<()> {
        HttpServer::new(|| {
            App::new()
                .service(users_controller::get_users)
                .service(cities_controller::get_cities)
        })
        .bind((addrs, port)).unwrap()
        .run()
        .await
    }
    match webserver(addrs, port) {
        Ok(..) => {
            println!("Webserver terminated");
        }
        Err(..) => {
            println!("Error during start webserver");
        }
    }
}
