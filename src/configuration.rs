#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {

    // initiate config reader
    let settings = config::Config::builder()
    // add config settings from yml file
    .add_source(config::File::new("configuration.yml", config::FileFormat::Yaml)).build()?;

    settings.try_deserialize::<Settings>()
}