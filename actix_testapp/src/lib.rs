use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use std::thread;
use std::net::TcpListener;

mod handlers;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {

    let res = HttpServer::new(|| {
        let app = App::new()
            .route("/", web::get().to(handlers::greet))
            .route("/health_check", web::get().to(handlers::health_check))
            .route("/{name}", web::get().to(handlers::greet));
        let id = thread::current().id();
        println!("App created: {id:?}");
        app
    })
    .listen(listener)?
    .run();
    Ok(res)
}
