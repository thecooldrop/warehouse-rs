use testcontainers::{Container, clients::Cli, images::postgres::Postgres, Docker, core::Port};
use rocket::{Rocket, Config, config::Environment};
use diesel::PgConnection;
use std::collections::HashMap;
use crate::controllers::product_category;
use crate::DbConn;

pub struct DatabaseMetadata<'a> {
    pub database_name: String,
    pub username: String,
    pub password: String,
    pub port: u32,
    pub url: String,
    pub db_container: Container<'a, Cli, Postgres>
}

pub fn start_database(docker: &Cli) -> DatabaseMetadata {
    let db = "testdb".to_string();
    let user = "testuser".to_string();
    let password = "testpassword".to_string();
    let mut environment_variables = HashMap::new();
    environment_variables.insert("POSTGRES_DB".to_string(), db.clone());
    environment_variables.insert("POSTGRES_USER".to_string(), user.clone());
    environment_variables.insert("POSTGRES_PASSWORD".to_string(), password.clone());
    let local_port = free_local_port().unwrap();
    let port_mapping = Port {
        local: local_port,
        internal: 5432
    };
    let postgres_image = Postgres::default()
        .with_env_vars(environment_variables)
        .with_mapped_port(port_mapping);
    let connection_str = format!("postgres://testuser:testpassword@localhost:{}/{}", local_port, db);
    let container = docker.run(postgres_image);
    DatabaseMetadata {
        database_name: db,
        username: user,
        password,
        port: local_port as u32,
        url: connection_str,
        db_container: container
    }
}

pub fn start_rocket_with_db(database_metadata: &DatabaseMetadata) -> Result<(Rocket, PgConnection),diesel_migrations::RunMigrationsError> {
    let connection: PgConnection = diesel::connection::Connection::establish(&database_metadata.url).unwrap();
    embed_migrations!("./migrations");
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout())?;

    let mut database_config = HashMap::new();
    let mut pg_database_config = HashMap::new();
    pg_database_config.insert("url", database_metadata.url.clone());
    pg_database_config.insert("port", database_metadata.port.to_string());
    database_config.insert("pgdatabase", pg_database_config);
    let rocket_config = Config::build(Environment::Development)
        .port(free_local_port().unwrap())
        .extra("databases", database_config)
        .finalize()
        .unwrap();

    let rocket = rocket::custom(rocket_config)
        .attach(DbConn::fairing())
        .mount("/productcategory", routes![product_category::get, product_category::get_all, product_category::post, product_category::delete, product_category::put]);
    Ok((rocket, connection))
}

fn free_local_port() -> Option<u16> {
    let socket = std::net::SocketAddrV4::new(std::net::Ipv4Addr::LOCALHOST, 0);
    std::net::TcpListener::bind(socket)
        .and_then(|listener| listener.local_addr())
        .map(|addr| addr.port())
        .ok()
}