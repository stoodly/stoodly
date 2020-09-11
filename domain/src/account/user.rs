use custom_error::custom_error;
use std::error::Error;
use uuid::Uuid;

#[derive(Debug, Clone, PartialOrd, PartialEq, Hash)]
pub struct User {
    pub id: Option<Uuid>,
    pub email: String,
    pub username: String,
    pub active: bool,
}

custom_error! {
    pub ValidationError
    IdIsPresent = "The 'User' entity must not have a value set for the unique identifier.",
    IdIsNone = "The 'User' entity must have a unique identifier.",
    InvalidId = "The provided ID is invalid",
}

pub trait Repository {
    fn add(&self, user: User) -> Result<User, Box<dyn Error>>;
    fn find_by_id(&self, id: Uuid) -> Result<Option<User>, Box<dyn Error>>;
    fn remove(&self, id: Uuid) -> Result<Option<User>, Box<dyn Error>>;
}

pub trait Service {
    fn create(&self, user: User) -> Result<User, Box<dyn Error>>;
    fn read(&self, id: Uuid) -> Result<Option<User>, Box<dyn Error>>;
    fn update(&self, user: User) -> Result<User, Box<dyn Error>>;
    fn delete(&self, id: Uuid) -> Result<Option<User>, Box<dyn Error>>;
}