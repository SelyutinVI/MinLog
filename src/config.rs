use std::time::Duration;

use crate::domain::Lifetime;

#[derive(serde::Deserialize, Debug)]
pub struct ConfigDataBase {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub user: Option<String>,
    pub password: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
pub struct ConfigLifetime {
    pub xs: Duration,
    pub s: Duration,
    pub m: Duration,
    pub l: Duration,
    pub xl: Duration,
    pub xxl: Duration,
}

#[derive(serde::Deserialize, Debug, Default)]
pub struct Config {
    pub database: Option<ConfigDataBase>,
    pub lifetime: Option<ConfigLifetime>,
}

impl Config {
    pub fn from_env() -> Self {
        let mut conf = Config {
            database: None,
            lifetime: None,
        };

        if let Ok(config) = envy::prefixed("DB_").from_env::<ConfigDataBase>() {
            conf.database = Some(config);
        };

        if let Ok(config) = envy::prefixed("LT_").from_env::<ConfigLifetime>() {
            conf.lifetime = Some(config);
        };

        println!("Config: {:#?}", conf);
        return conf;
    }

    pub fn from_file() -> Self {
        panic!("not implemented");
    }
}
