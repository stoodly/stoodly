use mongodb::sync::{Client, Collection, Database};
use mongodb::error::Error;

pub mod account;
pub mod status;

pub fn establish_mongodb_connection(database: &str, collection: &str) -> Collection {
    let uri = "mongodb://root:rootpassword@localhost:27017/";
    let client: Result<Client, Error> = Client::with_uri_str(uri);
    let database: Database = client.expect("client").database(database);
    database.collection(collection)
}