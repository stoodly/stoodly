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
