use lazy_static::lazy_static;
use status::post::{Post, Repository};
use std::error::Error;
use std::sync::{Mutex, MutexGuard};
use uuid::Uuid;

lazy_static! {
    static ref COLLECTION: Mutex<Vec<Post>> = Mutex::new(vec![]);
}

pub struct PostRepository {}

impl Repository for PostRepository {
    fn add(&self, post: Post) -> Result<Post, Box<dyn Error>> {
        let mut mut_post: Post = post.clone();
        if mut_post.id.is_none() {
            mut_post.id = Some(Uuid::new_v4());
        }
        let mut collection: MutexGuard<Vec<Post>> = COLLECTION.lock().unwrap();

        collection.push(mut_post.clone());
        Ok(mut_post)
    }

    fn find_by_id(&self, id: Uuid) -> Result<Option<Post>, Box<dyn Error>> {
        let collection: MutexGuard<Vec<Post>> = COLLECTION.lock().unwrap();
        let post_opt: Option<Post> = collection.iter().find(|post| post.id == Some(id)).cloned();
        Ok(post_opt)
    }

    fn find_all_by_team_id(&self, team_id: Uuid) -> Result<Vec<Post>, Box<dyn Error>> {
        let collection: MutexGuard<Vec<Post>> = COLLECTION.lock().unwrap();
        let filtered_posts: Vec<Post> = collection
            .iter()
            .filter(|post| post.team_id == team_id)
            .map(|post| post.clone())
            .collect();
        Ok(filtered_posts)
    }

    fn remove(&self, id: Uuid) -> Result<Option<Post>, Box<dyn Error>> {
        let mut collection: MutexGuard<Vec<Post>> = COLLECTION.lock().unwrap();
        let found_post: Option<Post> = collection.iter().find(|post| post.id == Some(id)).cloned();
        if found_post.is_some() {
            collection.retain(|post| post.id != found_post.clone().unwrap().id)
        }
        Ok(found_post)
    }
}
