use std::error::Error;

use domain::organization::team::{Repository, Service, Team, ValidationError};
use uuid::Uuid;

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
