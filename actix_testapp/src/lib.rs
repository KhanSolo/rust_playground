use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use std::thread;

mod appsettings;
mod handlers;

pub fn run() -> Result<Server, std::io::Error> {

    let app_settings = appsettings::load::load_settings()?;
    println!("appsettings loaded: {:?}", &app_settings);

    let res = HttpServer::new(|| {
        let app = App::new()
            .route("/", web::get().to(handlers::greet))
            .route("/health_check", web::get().to(handlers::health_check))
            .route("/{name}", web::get().to(handlers::greet));
        let id = thread::current().id();
        println!("App created: {id:?}");
        app
    })
    .bind(app_settings.baseurl)?
    .run();

    Ok(res)
}
