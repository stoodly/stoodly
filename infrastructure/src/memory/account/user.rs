use lazy_static::lazy_static;
use std::error::Error;
use std::sync::{Mutex, MutexGuard};
use uuid::Uuid;
use domain::account::user::{Repository, User};

lazy_static! {
    static ref COLLECTION: Mutex<Vec<User>> = Mutex::new(vec![]);
}

pub struct UserRepository {}

impl Repository for UserRepository {
    fn add(&self, user: User) -> Result<User, Box<dyn Error>> {
        let mut mut_user: User = user.clone();
        if mut_user.id.is_none() {
            mut_user.id = Some(Uuid::new_v4());
        }
        let mut collection: MutexGuard<Vec<User>> = COLLECTION.lock().unwrap();

        collection.push(mut_user.clone());
        Ok(mut_user)
    }

    fn find_by_id(&self, id: Uuid) -> Result<Option<User>, Box<dyn Error>> {
        let collection: MutexGuard<Vec<User>> = COLLECTION.lock().unwrap();
        let user_opt: Option<User> = collection.iter().find(|user| user.id == Some(id)).cloned();
        Ok(user_opt)
    }

    fn remove(&self, id: Uuid) -> Result<Option<User>, Box<dyn Error>> {
        let mut collection: MutexGuard<Vec<User>> = COLLECTION.lock().unwrap();
        let found_user: Option<User> = collection.iter().find(|user| user.id == Some(id)).cloned();
        if found_user.is_some() {
            collection.retain(|user| user.id != found_user.clone().unwrap().id)
        }
        Ok(found_user)
    }
}
