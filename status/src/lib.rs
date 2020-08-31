use custom_error::custom_error;
use std::error::Error;
use uuid::Uuid;

use account::user::{Service as UserService, User};
use organization::team::{Service as TeamService, Team};

use crate::post::{Post, Service as PostService};

pub mod post;

custom_error! {
    pub PermissionError
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

pub struct StatusService<P: PostService, U: UserService, T: TeamService> {
    pub post_service: P,
    pub user_service: U,
    pub team_service: T,
}

impl<P: PostService, U: UserService, T: TeamService> Service for StatusService<P, U, T> {
    fn create(&self, post: Post, user_id: Uuid) -> Result<Post, Box<dyn Error>> {
        fn permission(post: Post, user: User, team: Team) -> Result<Post, Box<dyn Error>> {
            let user_uuid: Uuid = user.id.expect("expected 'user' ID");
            let team_uuid: Uuid = team.id.expect("expected 'team' ID");
            let user_mismatch_check: bool = user_uuid != post.user_id;
            let not_team_member_check: bool = !team.members.contains(&user_uuid);
            let team_mismatch_check: bool = team_uuid != post.team_id;

            if user_mismatch_check {
                Err(PermissionError::UserMismatch.into())
            } else if not_team_member_check {
                Err(PermissionError::NotTeamMember.into())
            } else if team_mismatch_check {
                Err(PermissionError::TeamMismatch.into())
            } else {
                Ok(post)
            }
        }

        let user: User = match self.user_service.read(user_id)? {
            None => return Err(NotFoundError::UserNotFound.into()),
            Some(user) => user,
        };
        let team: Team = match self.team_service.read(post.team_id)? {
            None => return Err(NotFoundError::TeamNotFound.into()),
            Some(team) => team,
        };

        self.post_service.create(permission(post, user, team)?)
    }

    fn read(&self, id: Uuid, user_id: Uuid) -> Result<Option<Post>, Box<dyn Error>> {
        fn permission(post: Post, user: User, team: Team) -> Result<Option<Post>, Box<dyn Error>> {
            let user_uuid: Uuid = user.id.expect("expected 'user' ID");
            let team_uuid: Uuid = team.id.expect("expected 'team' ID");
            let user_mismatch_check: bool = user_uuid != post.user_id;
            let not_team_member_check: bool = !team.members.contains(&user_uuid);
            let team_mismatch_check: bool = team_uuid != post.team_id;

            if user_mismatch_check {
                Err(PermissionError::UserMismatch.into())
            } else if not_team_member_check {
                Err(PermissionError::NotTeamMember.into())
            } else if team_mismatch_check {
                Err(PermissionError::TeamMismatch.into())
            } else {
                Ok(Some(post))
            }
        }

        let post: Post = match self.post_service.read(id)? {
            None => return Ok(None),
            Some(post) => post,
        };
        let user: User = match self.user_service.read(user_id)? {
            None => return Err(NotFoundError::UserNotFound.into()),
            Some(user) => user,
        };
        let team: Team = match self.team_service.read(post.team_id)? {
            None => return Err(NotFoundError::TeamNotFound.into()),
            Some(team) => team,
        };

        permission(post, user, team)
    }

    fn update(&self, post: Post, user_id: Uuid) -> Result<Post, Box<dyn Error>> {
        fn permission(post: Post, user: User, team: Team) -> Result<Post, Box<dyn Error>> {
            let user_uuid: Uuid = user.id.expect("expected 'user' ID");
            let team_uuid: Uuid = team.id.expect("expected 'team' ID");
            let user_mismatch_check: bool = user_uuid != post.user_id;
            let not_team_member_check: bool = !team.members.contains(&user_uuid);
            let team_mismatch_check: bool = team_uuid != post.team_id;

            if user_mismatch_check {
                Err(PermissionError::UserMismatch.into())
            } else if not_team_member_check {
                Err(PermissionError::NotTeamMember.into())
            } else if team_mismatch_check {
                Err(PermissionError::TeamMismatch.into())
            } else {
                Ok(post)
            }
        }

        let user: User = match self.user_service.read(user_id)? {
            None => return Err(NotFoundError::UserNotFound.into()),
            Some(user) => user,
        };
        let team: Team = match self.team_service.read(post.team_id)? {
            None => return Err(NotFoundError::TeamNotFound.into()),
            Some(team) => team,
        };

        self.post_service.update(permission(post, user, team)?)
    }

    fn delete(&self, id: Uuid, user_id: Uuid) -> Result<Option<Post>, Box<dyn Error>> {
        fn permission(post: Post, user: User, team: Team) -> Result<Uuid, Box<dyn Error>> {
            let post_uuid: Uuid = post.id.expect("expected 'post' ID");
            let user_uuid: Uuid = user.id.expect("expected 'user' ID");
            let team_uuid: Uuid = team.id.expect("expected 'team' ID");
            let user_mismatch_check: bool = user_uuid != post.user_id;
            let not_team_member_check: bool = !team.members.contains(&user_uuid);
            let team_mismatch_check: bool = team_uuid != post.team_id;

            if user_mismatch_check {
                Err(PermissionError::UserMismatch.into())
            } else if not_team_member_check {
                Err(PermissionError::NotTeamMember.into())
            } else if team_mismatch_check {
                Err(PermissionError::TeamMismatch.into())
            } else {
                Ok(post_uuid)
            }
        }

        let post: Post = match self.post_service.read(id)? {
            None => return Ok(None),
            Some(post) => post,
        };
        let user: User = match self.user_service.read(user_id)? {
            None => return Err(NotFoundError::UserNotFound.into()),
            Some(user) => user,
        };
        let team: Team = match self.team_service.read(post.team_id)? {
            None => return Err(NotFoundError::TeamNotFound.into()),
            Some(team) => team,
        };

        self.post_service.delete(permission(post, user, team)?)
    }
}
