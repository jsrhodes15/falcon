#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
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
    // Initialize our config reader
    let settings = config::Config::builder()
        // add config values from a file names configuration.yaml
        .add_source(
            config::File::new("configuration.yaml", config::FileFormat::Yaml)
        )
        .build()?;
    // Try to convert the configuration values it reads into our Settings type
    settings.try_deserialize::<Settings>()
}