use std::collections::HashMap;

use html::{Head, Meta, Title};
use rocket::{
    config::{Environment, Value},
    fairing::AdHoc,
    Config, Rocket,
};

pub fn default_head(title: &str) -> Head {
    Head::default()
        .child(Title(format!("{} | Lovelace", title)))
        .child(
            Meta::default()
                .attribute(format!("rel"), format!("stylesheet"))
                .attribute(format!("href"), format!("/css/styles.css")),
        )
}

pub fn launch() -> Rocket {
    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();
    database_config.insert(
        "url",
        Value::from(
            std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost".to_string()),
        ),
    );
    databases.insert("postgres", Value::from(database_config));
    let config = Config::build(
        std::env::var("DEV")
            .map(|_| Environment::Development)
            .unwrap_or(Environment::Production),
    )
    .extra("databases", databases)
    .finalize()
    .unwrap();
    rocket::custom(config)
        .attach(crate::db::Database::fairing())
        .attach(AdHoc::on_attach(
            "Database Migrations",
            crate::db::run_migrations,
        ))
        .mount("/", routes![crate::index])
        .mount(
            "/auth",
            routes![
                crate::auth::login_page,
                crate::auth::login,
                crate::auth::register_page,
                crate::auth::register
            ],
        )
}
