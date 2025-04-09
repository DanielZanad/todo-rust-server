use log;
use std::{
    env::{self, VarError},
    time::Duration,
};

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub struct DatabaseSettings {
    username: String,
    password: String,
    database_name: String,
    database_host: String,
    pub database_connection: Option<DatabaseConnection>,
}

pub async fn get_configuration() -> Result<DatabaseSettings, VarError> {
    let username = env::var("DATABASE_USER");
    let password = env::var("DATABASE_PASSWORD");
    let database_name = env::var("DATABASE_NAME");
    let database_host = env::var("DATABASE_HOST");

    match (username, password, database_name, database_host) {
        (Ok(user), Ok(pass), Ok(db_name), Ok(host)) => {
            let mut database_settings = DatabaseSettings {
                username: user,
                password: pass,
                database_name: db_name.clone(),
                database_host: host,
                database_connection: None,
            };

            let mut opt = ConnectOptions::new(database_settings.connection_string());
            opt.max_connections(100)
                .min_connections(5)
                .connect_timeout(Duration::from_secs(8))
                .acquire_timeout(Duration::from_secs(8))
                .idle_timeout(Duration::from_secs(8))
                .max_lifetime(Duration::from_secs(8))
                .sqlx_logging(true)
                .sqlx_logging_level(log::LevelFilter::Info)
                .set_schema_search_path(db_name);

            let conn = Database::connect(opt).await.expect("Failed to connect");

            database_settings.database_connection = Some(conn);

            return Ok(database_settings);
        }
        (Err(error), _, _, _)
        | (_, Err(error), _, _)
        | (_, _, Err(error), _)
        | (_, _, _, Err(error)) => {
            println!("env error");
            return Err(error);
        }
    }
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}/{}?currentSchema=public",
            self.username, self.password, self.database_host, self.database_name
        )
    }
}
