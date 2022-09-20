use actix_web::{web, App, HttpRequest, HttpServer, Responder};

mod appsettings;
mod handlers;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Starting");
    let app_settings = appsettings::load::load_settings()?;
    println!("appsettings loaded");

    let res =
    HttpServer::new(|| {
        let app = App::new()
                    .route("/", web::get().to(handlers::greet))
                    .route("/{name}", web::get().to(handlers::greet));
            println!("App created");
            app
    })
    .bind(app_settings.baseurl)?
    .run()
    .await;
    println!("Completed");
    res
}

