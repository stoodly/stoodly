use std::error::Error;

use custom_error::custom_error;
use uuid::Uuid;

#[derive(Debug, Clone, PartialOrd, PartialEq, Hash)]
pub struct Team {
    pub id: Option<Uuid>,
    pub organization_id: Uuid,
    pub name: String,
    pub members: Vec<Uuid>,
}

custom_error! {
    pub ValidationError
    IdIsPresent = "The 'Team' entity must not have a value set for the unique identifier.",
    IdIsNone = "The 'Team' entity must have a unique identifier.",
    InvalidId = "The provided ID is invalid",
    InvalidOrganizationId = "The provided 'organization_id' is invalid",
}

pub trait Repository {
    fn add(&self, team: Team) -> Result<Team, Box<dyn Error>>;
    fn find_by_id(&self, id: Uuid) -> Result<Option<Team>, Box<dyn Error>>;
    fn find_all_by_organization_id(
        &self,
        organization_id: Uuid,
    ) -> Result<Vec<Team>, Box<dyn Error>>;
    fn remove(&self, id: Uuid) -> Result<Option<Team>, Box<dyn Error>>;
}

pub trait Service {
    fn create(&self, team: Team) -> Result<Team, Box<dyn Error>>;
    fn read(&self, id: Uuid) -> Result<Option<Team>, Box<dyn Error>>;
    fn list(&self, organization_id: Uuid) -> Result<Vec<Team>, Box<dyn Error>>;
    fn update(&self, team: Team) -> Result<Team, Box<dyn Error>>;
    fn delete(&self, id: Uuid) -> Result<Option<Team>, Box<dyn Error>>;
}

pub struct TeamService<R: Repository> {
    pub repository: R,
}

impl<R: Repository> Service for TeamService<R> {
    fn create(&self, team: Team) -> Result<Team, Box<dyn Error>> {
        fn validate(team: Team) -> Result<Team, Box<dyn Error>> {
            if team.id.is_some() {
                Err(ValidationError::IdIsPresent.into())
            } else {
                Ok(team)
            }
        }

        self.repository.add(validate(team)?)
    }

    fn read(&self, id: Uuid) -> Result<Option<Team>, Box<dyn Error>> {
        fn validate(id: Uuid) -> Result<Uuid, Box<dyn Error>> {
            if id.is_nil() {
                Err(ValidationError::InvalidId.into())
            } else {
                Ok(id)
            }
        }

        self.repository.find_by_id(validate(id)?)
    }

    fn list(&self, organization_id: Uuid) -> Result<Vec<Team>, Box<dyn Error>> {
        fn validate(organization_id: Uuid) -> Result<Uuid, Box<dyn Error>> {
            if organization_id.is_nil() {
                Err(ValidationError::InvalidOrganizationId.into())
            } else {
                Ok(organization_id)
            }
        }

        self.repository
            .find_all_by_organization_id(validate(organization_id)?)
    }

    fn update(&self, team: Team) -> Result<Team, Box<dyn Error>> {
        fn validate(team: Team) -> Result<Team, Box<dyn Error>> {
            if team.id.is_none() {
                Err(ValidationError::IdIsNone.into())
            } else if team.id.ok_or("expected ID")?.is_nil() {
                Err(ValidationError::InvalidId.into())
            } else {
                Ok(team)
            }
        }

        self.repository.add(validate(team)?)
    }

    fn delete(&self, id: Uuid) -> Result<Option<Team>, Box<dyn Error>> {
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
