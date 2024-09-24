use actix_web::{get, App, HttpServer, Responder, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
struct ApiResponse {
  message: String,
}

#[get("/api/ping")]
async fn ping() -> impl Responder {
  let response = ApiResponse {
    message: "pong".to_string(),
  };
  HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| App::new().service(ping))
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
