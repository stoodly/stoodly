use custom_error::custom_error;
use std::error::Error;
use uuid::Uuid;

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
