mod entities;
mod repositories;
mod controllers;
mod services;

use controllers::{users_controller, cities_controller};
use actix_web::{App, HttpServer};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let env = args.get(1)
        .expect("\n\nexample usage:\n1. ./program path/.env\n2. cargo run ./src/.env\nthe first argument must be the .env file\n\n");
    dotenv::from_path(&env)
        .expect(&*format!("file .env not exist in this location: {}", &env));
    println!("file {} successfully read", &env);
    let addrs = "0.0.0.0";
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
