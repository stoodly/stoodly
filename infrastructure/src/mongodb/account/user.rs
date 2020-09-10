use std::error::Error;

use domain::account::user::{Repository, User};
use mongodb::bson::{doc, Bson, Document};
use mongodb::{options::UpdateOptions, sync::Collection};
use uuid::Uuid;

pub struct UserRepository {
    pub collection: Collection,
}

impl Repository for UserRepository {
    fn add(&self, user: User) -> Result<User, Box<dyn Error>> {
        let mut mut_user: User = user.clone();
        let id: String = mut_user.id.get_or_insert(Uuid::new_v4()).to_string();
        self.collection.update_one(
            doc! {
                "_id": &id
            },
            doc! {
                "_id": &id,
                "email": &mut_user.email,
                "username": &mut_user.username,
                "active": &mut_user.active,
            },
            Some(UpdateOptions::builder().upsert(Some(true)).build()),
        )?;
        Ok(mut_user)
    }

    fn find_by_id(&self, id: Uuid) -> Result<Option<User>, Box<dyn Error>> {
        let document_opt: Option<Document> = self
            .collection
            .find_one(doc! { "_id": id.to_string() }, None)?;
        match document_opt {
            Some(document) => Ok(Some(document_to_user(document)?)),
            None => Ok(None),
        }
    }

    fn remove(&self, id: Uuid) -> Result<Option<User>, Box<dyn Error>> {
        let user: Result<Option<User>, Box<dyn Error>> = self.find_by_id(id);
        self.collection.delete_one(
            doc! {
                "_id": &id.to_string()
            },
            None,
        )?;
        user
    }
}

fn document_to_user(document: Document) -> Result<User, Box<dyn Error>> {
    let id: &str = document.get("_id").and_then(Bson::as_str).ok_or("_id")?;
    let email: String = document
        .get("email")
        .and_then(Bson::as_str)
        .map(|email| email.to_string())
        .ok_or("missing email")?;
    let username: String = document
        .get("username")
        .and_then(Bson::as_str)
        .map(|username| username.to_string())
        .ok_or("missing username")?;
    let active = document
        .get("active")
        .and_then(Bson::as_bool)
        .ok_or("missing active")?;
    let user: User = User {
        id: Some(Uuid::parse_str(id)?),
        email,
        username,
        active,
    };
    Ok(user)
}
