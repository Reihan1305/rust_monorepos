use mongodb::{Client, Database};

pub async fn create_client(mongo_url: &str, database_name: &str) -> Result<Database, mongodb::error::Error> {
    let client = Client::with_uri_str(mongo_url).await?;
    Ok(client.database(database_name))
}
