use domain::account::user::{Repository, User, ValidationError};
use std::error::Error;
use uuid::Uuid;

pub trait Service {
    fn create(&self, user: User) -> Result<User, Box<dyn Error>>;
    fn read(&self, id: Uuid) -> Result<Option<User>, Box<dyn Error>>;
    fn update(&self, user: User) -> Result<User, Box<dyn Error>>;
    fn delete(&self, id: Uuid) -> Result<Option<User>, Box<dyn Error>>;
}

pub struct UserService<R: Repository> {
    pub repository: R,
}

impl<R: Repository> Service for UserService<R> {
    fn create(&self, user: User) -> Result<User, Box<dyn Error>> {
        fn validate(user: User) -> Result<User, Box<dyn Error>> {
            if user.id.is_some() {
                Err(ValidationError::IdIsPresent.into())
            } else {
                Ok(user)
            }
        }

        self.repository.add(validate(user)?)
    }

    fn read(&self, id: Uuid) -> Result<Option<User>, Box<dyn Error>> {
        fn validate(id: Uuid) -> Result<Uuid, Box<dyn Error>> {
            if id.is_nil() {
                Err(ValidationError::InvalidId.into())
            } else {
                Ok(id)
            }
        }

        self.repository.find_by_id(validate(id)?)
    }

    fn update(&self, user: User) -> Result<User, Box<dyn Error>> {
        fn validate(user: User) -> Result<User, Box<dyn Error>> {
            if user.id.is_none() {
                Err(ValidationError::IdIsNone.into())
            } else if user.id.ok_or("expected ID")?.is_nil() {
                Err(ValidationError::InvalidId.into())
            } else {
                Ok(user)
            }
        }

        self.repository.add(validate(user)?)
    }

    fn delete(&self, id: Uuid) -> Result<Option<User>, Box<dyn Error>> {
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
