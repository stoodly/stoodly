use chrono::{DateTime, Utc};
use juniper::{GraphQLInputObject, GraphQLObject};
use uuid::Uuid;
use domain::status::post::Post;

#[derive(GraphQLObject)]
#[graphql(
    description = "A Post is the abstraction of a team's user status by providing the answers to three simple questions"
)]
pub struct QueryPost {
    pub id: Option<Uuid>,
    pub user_id: Uuid,
    pub team_id: Uuid,
    pub yesterday: Vec<String>,
    pub today: Vec<String>,
    pub blocker: Vec<String>,
    pub posted: DateTime<Utc>,
}

impl QueryPost {
    pub fn from_post(post: Post) -> Self {
        Self {
            id: post.id,
            user_id: post.user_id,
            team_id: post.team_id,
            yesterday: post.yesterday,
            today: post.today,
            blocker: post.blocker,
            posted: post.posted,
        }
    }
}

#[derive(GraphQLInputObject)]
#[graphql(
    description = "A Post is the abstraction of a team's user status by providing the answers to three simple questions"
)]
pub struct NewPost {
    pub user_id: Uuid,
    pub team_id: Uuid,
    pub yesterday: Vec<String>,
    pub today: Vec<String>,
    pub blocker: Vec<String>,
    pub posted: DateTime<Utc>,
}

impl NewPost {
    pub fn from_new_post(new_post: &Self) -> Post {
        Post {
            id: None,
            user_id: new_post.user_id,
            team_id: new_post.team_id,
            yesterday: new_post.yesterday.clone(),
            today: new_post.today.clone(),
            blocker: new_post.blocker.clone(),
            posted: new_post.posted,
        }
    }
}

#[derive(GraphQLInputObject)]
#[graphql(
    description = "A Post is the abstraction of a team's user status by providing the answers to three simple questions"
)]
pub struct UpdatePost {
    pub id: Uuid,
    pub user_id: Uuid,
    pub team_id: Uuid,
    pub yesterday: Vec<String>,
    pub today: Vec<String>,
    pub blocker: Vec<String>,
    pub posted: DateTime<Utc>,
}

impl UpdatePost {
    pub fn from_update_post(update_post: &Self) -> Post {
        Post {
            id: Some(update_post.id),
            user_id: update_post.user_id,
            team_id: update_post.team_id,
            yesterday: update_post.yesterday.clone(),
            today: update_post.today.clone(),
            blocker: update_post.blocker.clone(),
            posted: update_post.posted,
        }
    }
}
