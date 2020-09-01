use std::error::Error;

use mongodb::sync::{Client, Collection, Database};

pub mod account;
pub mod status;

pub fn establish_mongodb_connection(database: &str, collection: &str) -> Result<Collection, Box<dyn Error>> {
    let uri = "mongodb://root:rootpassword@localhost:27017/";
    let client: Client = Client::with_uri_str(uri)?;
    let database: Database = client.database(database);
    Ok(database.collection(collection))
}
