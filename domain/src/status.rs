use std::error::Error;

use custom_error::custom_error;
use uuid::Uuid;

use crate::status::post::Post;

pub mod post;

custom_error! {
    pub SecurityError
    NotTeamMember = "The 'User' entity is not a member of the retrieved 'Team' entity.",
    UserMismatch = "The user_id in the 'Post' entity does not match the id on the 'User' entity.",
    TeamMismatch = "The team_id in the 'Post' entity does not match the id on the 'Team' entity.",
}
custom_error! {
    pub NotFoundError
    PostNotFound = "The 'Post' entity not found.",
    UserNotFound = "The 'User' entity not found.",
    TeamNotFound = "The 'Team' entity not found.",
}

pub trait Service {
    fn create(&self, post: Post, user_id: Uuid) -> Result<Post, Box<dyn Error>>;
    fn read(&self, id: Uuid, user_id: Uuid) -> Result<Option<Post>, Box<dyn Error>>;
    fn update(&self, post: Post, user_id: Uuid) -> Result<Post, Box<dyn Error>>;
    fn delete(&self, id: Uuid, user_id: Uuid) -> Result<Option<Post>, Box<dyn Error>>;
}
