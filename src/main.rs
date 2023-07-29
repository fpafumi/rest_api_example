#[path = "repositories/user_repository.rs"] mod user_repository;
#[path = "entities/user.rs"] mod user_module;
use crate::user_repository::user_repository::*;
use crate::user_module::user_module::*;
//use serde_json::Result;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};


fn main() {
    println!("------------------- INIT ----------------");

    #[get("/users")]
    async fn get_users() -> impl Responder {
        let users: UserVec = read_csv("/home/furetto/Scrivania/progetti/applications/rest_api/raw/User.csv").unwrap();
        let json = serde_json::to_string(&users).unwrap();
        HttpResponse::Ok().body(json)
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
                .route("/hey", web::get().to(manual_hello))
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
    }
    webserver();


}
