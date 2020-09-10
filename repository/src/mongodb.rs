use std::error::Error;

use mongodb::sync::{Client, Collection, Database};

pub mod account;
pub mod organization;
pub mod status;

pub fn establish_mongodb_connection(
    database_name: &str,
    collection_name: &str,
) -> Result<Collection, Box<dyn Error>> {
    let uri = "mongodb://root:secret@localhost:27017/";
    let client: Client = Client::with_uri_str(uri)?;
    let database: Database = client.database(database_name);
    Ok(database.collection(collection_name))
}
