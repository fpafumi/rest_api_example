mod controllers;
mod entities;
mod repositories;
mod services;

use actix_web::{App, HttpServer};
use controllers::{cities_controller, users_controller};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let env = args.get(1)
        .expect("\n\nexample usage:\n1. ./program path/.env\n2. cargo run ./src/.env\nthe first argument must be the .env file\n\n");
    dotenv::from_path(&env).expect(&*format!("file .env not exist in this location: {}", &env));
    println!("file {} successfully read", &env);
    let addrs: String = std::env::var("ADDRS").expect("the ADDRS is not present in .env file");
    let addrs = &addrs as &str;
    let port: String = std::env::var("PORT").expect("the PORT is not present in .env file");
    let port = port.parse::<u16>().unwrap();
    println!("webserver is listening on {}:{}", &addrs, &port);

    #[actix_web::main]
    async fn webserver(addrs: &str, port: u16) -> std::io::Result<()> {
        HttpServer::new(|| {
            App::new()
                .service(users_controller::get_users)
                .service(cities_controller::get_cities)
        })
        .bind((addrs, port))
        .unwrap()
        .run()
        .await
    }
    match webserver(addrs, port) {
        Ok(__) => {
            println!("Webserver terminated");
        }
        Err(__) => {
            println!("Error during start webserver");
        }
    }
}
