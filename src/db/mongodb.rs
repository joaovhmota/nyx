use crate::types::Error;
use mongodb::{Client, Database};
use std::env;

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
}
