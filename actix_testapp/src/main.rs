use actix_testapp::run;
use std::net::TcpListener;

mod appsettings;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app_settings = appsettings::load_settings()?;
    let listener = TcpListener::bind(&app_settings.baseurl).expect("failed to bind");
    run(listener)?.await
}