use actix_web::{web, App, HttpServer};

mod appsettings;
mod handlers;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Starting");
    let app_settings = appsettings::load::load_settings()?;
    println!("appsettings loaded: {}", &app_settings.baseurl);

    let res =
    HttpServer::new(|| {
        let app = App::new()
                    .route("/", web::get().to(handlers::greet))
                    .route("/health_check", web::get().to(handlers::health_check))
                    .route("/{name}", web::get().to(handlers::greet))
                    ;
            println!("App created");
            app
    })
    .bind(app_settings.baseurl)?
    .run()
    .await;
    println!("Completed");
    res
}

