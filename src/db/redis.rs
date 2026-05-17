use crate::types::Error;
use redis::Client;
use redis::aio::MultiplexedConnection;
use std::env;

pub struct NyxRedis {}

impl NyxRedis {
    pub async fn get_client() -> Result<MultiplexedConnection, Error> {
        let client = Client::open(env::var("REDIS_URL").unwrap())?;
        let con = client.get_multiplexed_async_connection().await?;

        Ok(con)
    }
}
