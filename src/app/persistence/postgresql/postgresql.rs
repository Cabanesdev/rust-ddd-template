use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::collections::HashMap;
use std::env;
use phf::{phf_map};

pub fn create_client() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("DATABASE_URL {}", database_url);
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

static CLIENTS: phf::Map<&'static str, &'static PgConnection> = phf_map!{};

pub fn get_client(client: &String) -> Option<&PgConnection> {
    if CLIENTS.contains_key(client) {
        return CLIENTS.get(client);
    }else {
        let new_client = create_client();
        CLIENTS.insert(client.to_string(), new_client);
        return CLIENTS.get(client);
    }

    // if let Some(postgres_client) = CLIENTS.get(client) {
    //     return postgres_client;
    // } else {
    //     CLIENTS.insert(&client.to_string(), create_client());
    //     return CLIENTS.get(&client).clone();
    // }
}
