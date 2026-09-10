const DEFAULT_PORT: u16 = 8080;
const DEFAULT_MAX_CONNECTIONS: u32 = 5;

#[derive(Debug)]
pub struct ConfigError(String);

impl std::fmt::Display for ConfigError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for ConfigError {}

#[derive(Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub database_url: String,
    pub max_connections: u32,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let database_url = std::env::var("DATABASE_URL").map_err(|_| {
            ConfigError("DATABASE_URL must be set to a PostgreSQL connection string".to_owned())
        })?;

        Ok(Self {
            port: numeric_env("PORT").unwrap_or(DEFAULT_PORT),
            database_url,
            max_connections: numeric_env("DATABASE_MAX_CONNECTIONS")
                .unwrap_or(DEFAULT_MAX_CONNECTIONS),
        })
    }
}

fn numeric_env<T: std::str::FromStr>(name: &str) -> Option<T> {
    std::env::var(name).ok().and_then(|value| value.parse().ok())
}
