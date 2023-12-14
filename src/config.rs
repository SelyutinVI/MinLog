#[derive(serde::Deserialize, Debug)]
pub(crate) struct ConfigDataBase {
    pub(crate) host: Option<String>,
    pub(crate) port: Option<u16>,
    pub(crate) user: Option<String>,
    pub(crate) password: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct Config {
    pub(crate) database: Option<ConfigDataBase>,
}

impl Config {
    pub(crate) fn new() -> Self {
        let mut conf = Config { database: None };

        if let Ok(config) = envy::prefixed("DB_").from_env::<ConfigDataBase>() {
            conf.database = Some(config);
        };

        println!("Config: {:#?}", conf);
        return conf;
    }
}
