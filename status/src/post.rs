use chrono::DateTime;
use chrono::Utc;
use custom_error::custom_error;
use std::error::Error;
use uuid::Uuid;

#[derive(Debug, Clone, PartialOrd, PartialEq, Hash)]
pub struct Post {
    pub id: Option<Uuid>,
    pub user_id: Uuid,
    pub team_id: Uuid,
    pub yesterday: Vec<String>,
    pub today: Vec<String>,
    pub blocker: Vec<String>,
    pub posted: DateTime<Utc>,
}

custom_error! {
    pub PostError
    IdIsPresent = "The 'Post' entity must not have a value set for the unique identifier.",
    IdIsNone = "The 'Post' entity must have a unique identifier.",
    InvalidId = "The provided ID is invalid",
}

pub trait Repository {
    fn add(&self, post: Post) -> Result<Post, Box<dyn Error>>;
    fn find_by_id(&self, id: Uuid) -> Result<Option<Post>, Box<dyn Error>>;
    fn remove(&self, id: Uuid) -> Result<Option<Post>, Box<dyn Error>>;
}

pub trait Service {
    fn create(&self, post: Post) -> Result<Post, Box<dyn Error>>;
    fn read(&self, id: Uuid) -> Result<Option<Post>, Box<dyn Error>>;
    fn update(&self, post: Post) -> Result<Post, Box<dyn Error>>;
    fn delete(&self, id: Uuid) -> Result<Option<Post>, Box<dyn Error>>;
}

pub struct PostService<R: Repository> {
    pub repository: R,
}

impl<R: Repository> Service for PostService<R> {
    fn create(&self, post: Post) -> Result<Post, Box<dyn Error>> {
        fn validate(post: Post) -> Result<Post, Box<dyn Error>> {
            if post.id.is_some() {
                Err(PostError::IdIsPresent.into())
            } else {
                Ok(post)
            }
        }

        self.repository.add(validate(post)?)
    }

    fn read(&self, id: Uuid) -> Result<Option<Post>, Box<dyn Error>> {
        fn validate(id: Uuid) -> Result<Uuid, Box<dyn Error>> {
            if id.is_nil() {
                Err(PostError::InvalidId.into())
            } else {
                Ok(id)
            }
        }

        self.repository.find_by_id(validate(id)?)
    }

    fn update(&self, post: Post) -> Result<Post, Box<dyn Error>> {
        fn validate(post: Post) -> Result<Post, Box<dyn Error>> {
            if post.id.is_none() {
                Err(PostError::IdIsNone.into())
            } else if post.id.ok_or("expected ID")?.is_nil() {
                Err(PostError::InvalidId.into())
            } else {
                Ok(post)
            }
        }

        self.repository.add(validate(post)?)
    }

    fn delete(&self, id: Uuid) -> Result<Option<Post>, Box<dyn Error>> {
        fn validate(id: Uuid) -> Result<Uuid, Box<dyn Error>> {
            if id.is_nil() {
                Err(PostError::InvalidId.into())
            } else {
                Ok(id)
            }
        }

        self.repository.remove(validate(id)?)
    }
}
