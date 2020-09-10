use std::error::Error;

use custom_error::custom_error;
use uuid::Uuid;

use domain::account::user::User;
use domain::organization::team::Team;
use domain::status::{NotFoundError, SecurityError};
use domain::status::post::Post;

use crate::account::user::Service as UserService;
use crate::organization::team::Service as TeamService;
use crate::status::post::Service as PostService;

pub mod post;

pub trait Service {
    fn create(&self, post: Post, user_id: Uuid) -> Result<Post, Box<dyn Error>>;
    fn read(&self, id: Uuid, user_id: Uuid) -> Result<Option<Post>, Box<dyn Error>>;
    fn update(&self, post: Post, user_id: Uuid) -> Result<Post, Box<dyn Error>>;
    fn delete(&self, id: Uuid, user_id: Uuid) -> Result<Option<Post>, Box<dyn Error>>;
}

pub struct StatusService<P: PostService, U: UserService, T: TeamService> {
    pub post_service: P,
    pub user_service: U,
    pub team_service: T,
}

impl<P: PostService, U: UserService, T: TeamService> Service for StatusService<P, U, T> {
    fn create(&self, post: Post, user_id: Uuid) -> Result<Post, Box<dyn Error>> {
        let user: User = self
            .user_service
            .read(user_id)?
            .ok_or(NotFoundError::UserNotFound)?;
        let team: Team = self
            .team_service
            .read(post.team_id)?
            .ok_or(NotFoundError::TeamNotFound)?;

        self.post_service
            .create(security_check(post, user, team, false)?)
    }

    fn read(&self, id: Uuid, user_id: Uuid) -> Result<Option<Post>, Box<dyn Error>> {
        let post: Post = match self.post_service.read(id)? {
            None => return Ok(None),
            Some(post) => post,
        };
        let user: User = self
            .user_service
            .read(user_id)?
            .ok_or(NotFoundError::UserNotFound)?;
        let team: Team = self
            .team_service
            .read(post.team_id)?
            .ok_or(NotFoundError::TeamNotFound)?;

        security_check(post, user, team, true).map(|p| Some(p))
    }

    fn update(&self, post: Post, user_id: Uuid) -> Result<Post, Box<dyn Error>> {
        let user: User = self
            .user_service
            .read(user_id)?
            .ok_or(NotFoundError::UserNotFound)?;
        let team: Team = self
            .team_service
            .read(post.team_id)?
            .ok_or(NotFoundError::TeamNotFound)?;

        self.post_service
            .update(security_check(post, user, team, false)?)
    }

    fn delete(&self, id: Uuid, user_id: Uuid) -> Result<Option<Post>, Box<dyn Error>> {
        let post: Post = match self.post_service.read(id)? {
            None => return Ok(None),
            Some(post) => post,
        };
        let user: User = self
            .user_service
            .read(user_id)?
            .ok_or(NotFoundError::UserNotFound)?;
        let team: Team = self
            .team_service
            .read(post.team_id)?
            .ok_or(NotFoundError::TeamNotFound)?;

        self.post_service.delete(
            security_check(post, user, team, false)?
                .id
                .ok_or("expected 'post' ID")?,
        )
    }
}

fn security_check(
    post: Post,
    user: User,
    team: Team,
    read_only: bool,
) -> Result<Post, Box<dyn Error>> {
    let user_uuid: Uuid = user.id.ok_or("expected 'user' ID")?;
    let team_uuid: Uuid = team.id.ok_or("expected 'team' ID")?;
    let user_mismatch_check: bool = user_uuid != post.user_id || read_only;
    let not_team_member_check: bool = !team.members.contains(&post.user_id);
    let team_mismatch_check: bool = team_uuid != post.team_id;

    if user_mismatch_check {
        Err(SecurityError::UserMismatch.into())
    } else if not_team_member_check {
        Err(SecurityError::NotTeamMember.into())
    } else if team_mismatch_check {
        Err(SecurityError::TeamMismatch.into())
    } else {
        Ok(post)
    }
}
