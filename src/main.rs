mod entities;
mod repositories;
use entities::user::UserVec;
use entities::city::CityVec;
use repositories::user_repository;
use repositories::city_repository;

use actix_web::{
    get, http::header::ContentType, post, web, App, HttpResponse, HttpServer, Responder,
};

fn main() {
    println!("------------------- INIT ----------------");

    #[get("/users")]
    async fn get_users() -> impl Responder {
        let users: UserVec =
            user_repository::get_users("/home/furetto/Scrivania/progetti/applications/rest_api/raw/user.csv")
                .unwrap();
        let json = serde_json::to_string(&users).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(json)
    }

    #[get("/cities")]
    async fn get_cities() -> impl Responder {
        let cities: CityVec =
            city_repository::get_cities("/home/furetto/Scrivania/progetti/applications/rest_api/raw/city.csv")
                .unwrap();
        let json = serde_json::to_string(&cities).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(json)
    }

    #[get("/")]
    async fn hello() -> impl Responder {
        HttpResponse::Ok().body("Ciao tigre!")
    }

    #[post("/echo")]
    async fn echo(req_body: String) -> impl Responder {
        HttpResponse::Ok().body(req_body)
    }

    async fn manual_hello() -> impl Responder {
        HttpResponse::Ok().body("Hey there!")
    }

    #[actix_web::main]
    async fn webserver() -> std::io::Result<()> {
        HttpServer::new(|| {
            App::new()
                .service(hello)
                .service(echo)
                .service(get_users)
                .service(get_cities)
                .route("/hey", web::get().to(manual_hello))
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
    }
    match webserver() {
        Ok(..) => {
            println!("Webserver terminated");
        }
        Err(..) => {
            println!("Error during start webserver");
        }
    }
}
