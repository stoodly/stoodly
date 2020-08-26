use std::error::Error;

use mongodb::bson::{doc, Bson, Document};
use mongodb::{options::UpdateOptions, sync::Collection};
use uuid::Uuid;

use chrono::{TimeZone, Utc};

use status::post::{Post, Repository};

pub struct PostRepository {
    pub collection: Collection,
}

impl Repository for PostRepository {
    fn add(&self, post: Post) -> Result<Post, Box<dyn Error>> {
        let mut mut_post: Post = post.clone();
        let id: &mut Uuid = mut_post.id.get_or_insert(Uuid::new_v4());
        let upsert_enabled: bool = true;
        self.collection.update_one(
            doc! {
                "_id": &id.to_string()
            },
            doc! {
                "_id": &id.to_string(),
                "user_id": mut_post.user_id.to_string(),
                "team_id": mut_post.team_id.to_string(),
                "yesterday": &mut_post.yesterday,
                "today": &mut_post.today,
                "blocker": &mut_post.blocker,
                "posted": mut_post.posted.timestamp_millis()
            },
            Some(
                UpdateOptions::builder()
                    .upsert(Some(upsert_enabled))
                    .build(),
            ),
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
    let id: &str = document.get("_id").and_then(Bson::as_str).expect("_id");
    let user_id: &str = document
        .get("user_id")
        .and_then(Bson::as_str)
        .expect("user_id");
    let team_id: &str = document
        .get("team_id")
        .and_then(Bson::as_str)
        .expect("team_id");
    let yesterday: Vec<String> = document
        .get("yesterday")
        .and_then(Bson::as_array)
        .expect("yesterday")
        .iter()
        .map(|bson| bson.as_str().expect("yesterday's value").to_string())
        .collect();
    let today: Vec<String> = document
        .get("today")
        .and_then(Bson::as_array)
        .expect("today")
        .iter()
        .map(|bson| bson.as_str().expect("today's value").to_string())
        .collect();
    let blocker: Vec<String> = document
        .get("blocker")
        .and_then(Bson::as_array)
        .expect("blocker")
        .iter()
        .map(|bson| bson.as_str().expect("blocker's value").to_string())
        .collect();
    let posted = document
        .get("posted")
        .and_then(Bson::as_i64)
        .expect("posted");
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
