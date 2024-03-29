use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;
use std::thread;

mod handlers;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let res = HttpServer::new(|| {
        let app = App::new()
            //.route("/", web::get().to(handlers::greet))
            .route("/health_check", web::get().to(handlers::health_check))
            //.route("/{name}", web::get().to(handlers::greet));
            .route("/subscriptions", web::post().to(handlers::subscribe));
        let id = thread::current().id();
        println!("App created: {id:?}");
        app
    })
    .listen(listener)?
    .run();
    Ok(res)
}
