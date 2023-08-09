use config::Config;
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

#[derive(Deserialize, Debug)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ApplicationSettings {
    pub host: String,
    pub port: u16,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let cwd = std::env::current_dir().expect("Failed to get current directory");
    let config_dir = cwd.join("configuration");

    let settings =
        Config::builder().add_source(config::File::from(config_dir.join("base")).required(true));

    let app_env: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse environment");

    let settings =
        settings.add_source(config::File::from(config_dir.join(app_env.as_str())).required(true));

    settings
        .build()
        .expect("Failed to build configuration")
        .try_deserialize()
}

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}
impl TryFrom<String> for Environment {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            _ => Err(format!(
                "Invalid environment {}. Use either `local` or `production`",
                value
            )),
        }
    }
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database_name
        ))
    }
    pub fn connection_string_without_db(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port
        ))
    }
}

impl ApplicationSettings {
    pub fn server_url(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
