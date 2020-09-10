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
    pub ValidationError
    IdIsPresent = "The 'Post' entity must not have a value set for the unique identifier.",
    IdIsNone = "The 'Post' entity must have a unique identifier.",
    InvalidId = "The provided 'id' is invalid",
    InvalidTeamId = "The provided 'team_id' is invalid",
}

pub trait Repository {
    fn add(&self, post: Post) -> Result<Post, Box<dyn Error>>;
    fn find_by_id(&self, id: Uuid) -> Result<Option<Post>, Box<dyn Error>>;
    fn find_all_by_team_id(&self, team_id: Uuid) -> Result<Vec<Post>, Box<dyn Error>>;
    fn remove(&self, id: Uuid) -> Result<Option<Post>, Box<dyn Error>>;
}
