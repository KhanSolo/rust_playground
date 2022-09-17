use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize)]
struct Appsettings {
    baseurl: String,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    
    // Open the file in read-only mode with buffer.
    let file = File::open("appsettings.json")?;
    let reader = BufReader::new(file);
    let serialized:Appsettings = serde_json::from_reader(reader)?;
    //let addr = "0.0.0.0:8000";
    let addr = serialized.baseurl;

    HttpServer::new(|| {
        App::new()
        .route("/", web::get().to(greet))
        .route("/{name}/", web::get().to(greet))
    })
    .bind(addr)?
    .run()
    .await
}

async fn greet(req: HttpRequest)-> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello, {}!", &name)
}
