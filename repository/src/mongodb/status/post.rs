use std::error::Error;
use std::slice::Iter;

use chrono::{TimeZone, Utc};
use mongodb::bson::{doc, Bson, Document};
use mongodb::sync::Cursor;
use mongodb::{options::UpdateOptions, sync::Collection};
use uuid::Uuid;

use status::post::{Post, Repository};

pub struct PostRepository {
    pub collection: Collection,
}

impl Repository for PostRepository {
    fn add(&self, post: Post) -> Result<Post, Box<dyn Error>> {
        let mut mut_post: Post = post.clone();
        let id: String = mut_post.id.get_or_insert(Uuid::new_v4()).to_string();
        self.collection.update_one(
            doc! {
                "_id": &id
            },
            doc! {
                "_id": &id,
                "user_id": mut_post.user_id.to_string(),
                "team_id": mut_post.team_id.to_string(),
                "yesterday": &mut_post.yesterday,
                "today": &mut_post.today,
                "blocker": &mut_post.blocker,
                "posted": mut_post.posted.timestamp_millis()
            },
            Some(UpdateOptions::builder().upsert(Some(true)).build()),
        )?;
        Ok(mut_post)
    }

    fn find_by_id(&self, id: Uuid) -> Result<Option<Post>, Box<dyn Error>> {
        let document_opt: Option<Document> = self
            .collection
            .find_one(doc! { "_id": id.to_string() }, None)?;
        match document_opt {
            Some(document) => Ok(Some(document_to_post(document)?)),
            None => Ok(None),
        }
    }

    fn find_all_by_team_id(&self, team_id: Uuid) -> Result<Vec<Post>, Box<dyn Error>> {
        Ok(self
            .collection
            .find(doc! { "team_id": team_id.to_string() }, None)?
            .into_iter()
            .filter_map(Result::ok)
            .map(document_to_post)
            .filter_map(Result::ok)
            .collect())
    }

    fn remove(&self, id: Uuid) -> Result<Option<Post>, Box<dyn Error>> {
        let post: Result<Option<Post>, Box<dyn Error>> = self.find_by_id(id);
        self.collection.delete_one(
            doc! {
                "_id": &id.to_string()
            },
            None,
        )?;
        post
    }
}

fn document_to_post(document: Document) -> Result<Post, Box<dyn Error>> {
    let id: &str = document.get("_id").and_then(Bson::as_str).ok_or("_id")?;
    let user_id: &str = document
        .get("user_id")
        .and_then(Bson::as_str)
        .ok_or("user_id")?;
    let team_id: &str = document
        .get("team_id")
        .and_then(Bson::as_str)
        .ok_or("team_id")?;
    let yesterday: Vec<String> = document
        .get("yesterday")
        .and_then(Bson::as_array)
        .ok_or("yesterday")?
        .iter()
        .map(|bson| bson.as_str().map(|v| v.to_string()))
        .flatten()
        .collect();
    let today: Vec<String> = document
        .get("today")
        .and_then(Bson::as_array)
        .ok_or("today")?
        .iter()
        .map(|bson| bson.as_str().map(|v| v.to_string()))
        .flatten()
        .collect();
    let blocker: Vec<String> = document
        .get("blocker")
        .and_then(Bson::as_array)
        .ok_or("blocker")?
        .iter()
        .map(|bson| bson.as_str().map(|v| v.to_string()))
        .flatten()
        .collect();
    let posted = document
        .get("posted")
        .and_then(Bson::as_i64)
        .ok_or("posted")?;
    let post: Post = Post {
        id: Some(Uuid::parse_str(id)?),
        user_id: Uuid::parse_str(user_id)?,
        team_id: Uuid::parse_str(team_id)?,
        today,
        yesterday,
        blocker,
        posted: Utc.timestamp_millis(posted),
    };
    Ok(post)
}
