use domain::status::post::{Post, Repository, ValidationError, Service};
use std::error::Error;
use uuid::Uuid;

pub struct PostService<R: Repository> {
    pub repository: R,
}

impl<R: Repository> Service for PostService<R> {
    fn create(&self, post: Post) -> Result<Post, Box<dyn Error>> {
        fn validate(post: Post) -> Result<Post, Box<dyn Error>> {
            if post.id.is_some() {
                Err(ValidationError::IdIsPresent.into())
            } else {
                Ok(post)
            }
        }

        self.repository.add(validate(post)?)
    }

    fn read(&self, id: Uuid) -> Result<Option<Post>, Box<dyn Error>> {
        fn validate(id: Uuid) -> Result<Uuid, Box<dyn Error>> {
            if id.is_nil() {
                Err(ValidationError::InvalidId.into())
            } else {
                Ok(id)
            }
        }

        self.repository.find_by_id(validate(id)?)
    }

    fn list(&self, team_id: Uuid) -> Result<Vec<Post>, Box<dyn Error>> {
        fn validate(team_id: Uuid) -> Result<Uuid, Box<dyn Error>> {
            if team_id.is_nil() {
                Err(ValidationError::InvalidTeamId.into())
            } else {
                Ok(team_id)
            }
        }

        self.repository.find_all_by_team_id(validate(team_id)?)
    }

    fn update(&self, post: Post) -> Result<Post, Box<dyn Error>> {
        fn validate(post: Post) -> Result<Post, Box<dyn Error>> {
            if post.id.is_none() {
                Err(ValidationError::IdIsNone.into())
            } else if post.id.ok_or("expected ID")?.is_nil() {
                Err(ValidationError::InvalidId.into())
            } else {
                Ok(post)
            }
        }

        self.repository.add(validate(post)?)
    }

    fn delete(&self, id: Uuid) -> Result<Option<Post>, Box<dyn Error>> {
        fn validate(id: Uuid) -> Result<Uuid, Box<dyn Error>> {
            if id.is_nil() {
                Err(ValidationError::InvalidId.into())
            } else {
                Ok(id)
            }
        }

        self.repository.remove(validate(id)?)
    }
}
