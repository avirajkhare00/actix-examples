use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::Mutex;

#[derive(Deserialize, Serialize, Clone)]
struct UserProfile {
    username: String,
    email: String,
    bio: String,
}

#[derive(Clone)]
struct AppState {
    data: Vec<UserProfile>,
}

// Function to read the JSON file and parse it into a Vec<UserProfile>
fn read_json_file() -> Vec<UserProfile> {
    let file_content = fs::read_to_string("data.json").expect("Failed to read file");
    serde_json::from_str(&file_content).expect("Failed to parse JSON")
}

#[get("/api/profile/{username}")]
async fn get_profile(data: web::Data<Mutex<AppState>>, username: web::Path<String>) -> impl Responder {
    let state = data.lock().unwrap();
    let username = username.into_inner();

    // Search for the user by username
    if let Some(user) = state.data.iter().find(|user| user.username == username) {
        HttpResponse::Ok().json(user.clone())
    } else {
        HttpResponse::NotFound().body("User not found")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = AppState {
        data: read_json_file(),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Mutex::new(app_data.clone())))
            .service(get_profile)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
