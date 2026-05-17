use crate::types::Error;
use mongodb::Client;
use std::env;

pub struct NyxMongo {}

impl NyxMongo {
    pub async fn get_client() -> Result<Client, Error> {
        let client = Client::with_uri_str(env::var("MONGODB_URL").unwrap()).await?;

        Ok(client)
    }
}
