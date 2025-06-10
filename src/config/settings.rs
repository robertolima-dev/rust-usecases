use serde::Deserialize;
use std::env;
use std::net::IpAddr;
use std::str::FromStr;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct DatabaseSettings {
    #[validate(length(min = 1, message = "DATABASE_URL não pode estar vazia"))]
    pub url: String,
    #[validate(range(
        min = 1,
        max = 100,
        message = "Número de conexões deve estar entre 1 e 100"
    ))]
    pub max_connections: u32,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct JwtSettings {
    #[validate(length(min = 32, message = "JWT_SECRET deve ter pelo menos 32 caracteres"))]
    pub secret: String,
    #[validate(range(
        min = 300,
        max = 86400,
        message = "JWT_EXPIRES_IN deve estar entre 300 e 86400 segundos"
    ))]
    pub expires_in: u64,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct ServerSettings {
    #[validate(custom = "validate_ip")]
    pub host: IpAddr,
    #[validate(range(min = 1, max = 65535, message = "Porta deve estar entre 1 e 65535"))]
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct Settings {
    #[validate]
    pub database: DatabaseSettings,
    #[validate]
    pub jwt: JwtSettings,
    #[validate]
    pub server: ServerSettings,
    pub environment: Environment,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub enum Environment {
    Development,
    Testing,
    Production,
}

impl FromStr for Environment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "development" => Ok(Environment::Development),
            "testing" => Ok(Environment::Testing),
            "production" => Ok(Environment::Production),
            _ => Err(format!("Ambiente inválido: {}", s)),
        }
    }
}

fn validate_ip(ip: &IpAddr) -> Result<(), validator::ValidationError> {
    if ip.is_unspecified() {
        let mut err = validator::ValidationError::new("invalid_ip");
        err.message = Some("IP não pode ser 0.0.0.0".into());
        return Err(err);
    }
    Ok(())
}

#[allow(dead_code)]
impl Settings {
    pub fn load() -> Result<Self, String> {
        let environment = env::var("APP_ENVIRONMENT")
            .unwrap_or_else(|_| "development".to_string())
            .parse()?;

        let settings = Settings {
            database: DatabaseSettings {
                url: env::var("DATABASE_URL").map_err(|_| "DATABASE_URL não definida")?,
                max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                    .unwrap_or_else(|_| "5".to_string())
                    .parse()
                    .map_err(|_| "DATABASE_MAX_CONNECTIONS deve ser um número")?,
            },
            jwt: JwtSettings {
                secret: env::var("JWT_SECRET").map_err(|_| "JWT_SECRET não definida")?,
                expires_in: env::var("JWT_EXPIRES_IN")
                    .unwrap_or_else(|_| "86400".to_string())
                    .parse()
                    .map_err(|_| "JWT_EXPIRES_IN deve ser um número")?,
            },
            server: ServerSettings {
                host: env::var("SERVER_HOST")
                    .unwrap_or_else(|_| "127.0.0.1".to_string())
                    .parse()
                    .map_err(|_| "SERVER_HOST inválido")?,
                port: env::var("SERVER_PORT")
                    .unwrap_or_else(|_| "8080".to_string())
                    .parse()
                    .map_err(|_| "SERVER_PORT deve ser um número")?,
            },
            environment,
        };

        settings
            .validate()
            .map_err(|e| format!("Configurações inválidas: {}", e))?;
        Ok(settings)
    }

    pub fn is_development(&self) -> bool {
        self.environment == Environment::Development
    }

    pub fn is_production(&self) -> bool {
        self.environment == Environment::Production
    }

    pub fn is_testing(&self) -> bool {
        self.environment == Environment::Testing
    }
}
