use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub iface_name: String,
}

pub fn load() -> Settings {
    config::Config::builder()
        .add_source(config::File::with_name("config"))
        .build()
        .expect("failed to load config")
        .try_deserialize()
        .expect("invalid config")
}