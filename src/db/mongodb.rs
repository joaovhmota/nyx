use crate::types::Error;
use logfy::{critical, information, success};
use mongodb::{Client, Database};
use std::env;
use std::process::exit;

pub struct NyxMongo {}

impl NyxMongo {
    pub async fn get_client() -> Result<Client, Error> {
        let client = Client::with_uri_str(env::var("MONGODB_URL").unwrap()).await?;

        Ok(client)
    }

    pub async fn get_db() -> Result<Database, Error> {
        let client = Self::get_client().await?;
        let db = client.database("nyx");

        Ok(db)
    }

    pub async fn test_connection() {
        information!("Connecting to MongoDB…");
        let mongo_client = NyxMongo::get_client().await;

        match mongo_client {
            Ok(_) => {
                success!("Successfully connected to MongoDB");
            }
            Err(err) => {
                critical!("Could not connect to MongoDB. Reason: {}", err);
                exit(1);
            }
        }
    }
}
