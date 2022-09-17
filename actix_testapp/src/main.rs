use actix_web::{web, App, HttpRequest, HttpServer, Responder};

mod appsettings;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app_settings = appsettings::load::load_settings()?;

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind(app_settings.baseurl)?
    .run()
    .await
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello, {}!", &name)
}
