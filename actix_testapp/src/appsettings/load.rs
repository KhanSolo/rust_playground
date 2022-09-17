use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize)]
pub struct Appsettings {
    pub baseurl: String,
}

pub fn load_settings() -> std::io::Result<Appsettings> {
    let file = File::open("appsettings.json")?;
    let reader = BufReader::new(file);
    let app_settings: Appsettings = serde_json::from_reader(reader)?;
    Ok(app_settings)
}
