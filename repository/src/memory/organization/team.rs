use lazy_static::lazy_static;
use organization::team::{Repository, Team};
use std::error::Error;
use std::sync::{Mutex, MutexGuard};
use uuid::Uuid;

lazy_static! {
    static ref COLLECTION: Mutex<Vec<Team>> = Mutex::new(vec![]);
}

pub struct TeamRepository {}

impl Repository for TeamRepository {
    fn add(&self, team: Team) -> Result<Team, Box<dyn Error>> {
        let mut mut_team: Team = team.clone();
        if mut_team.id.is_none() {
            mut_team.id = Some(Uuid::new_v4());
        }
        let mut collection: MutexGuard<Vec<Team>> = COLLECTION.lock().unwrap();

        collection.push(mut_team.clone());
        Ok(mut_team)
    }

    fn find_by_id(&self, id: Uuid) -> Result<Option<Team>, Box<dyn Error>> {
        let collection: MutexGuard<Vec<Team>> = COLLECTION.lock().unwrap();
        let team_opt: Option<Team> = collection.iter().find(|team| team.id == Some(id)).cloned();
        Ok(team_opt)
    }

    fn find_all_by_organization_id(
        &self,
        organization_id: Uuid,
    ) -> Result<Vec<Team>, Box<dyn Error>> {
        let collection: MutexGuard<Vec<Team>> = COLLECTION.lock().unwrap();
        let filtered_teams: Vec<Team> = collection
            .iter()
            .filter(|team| team.organization_id == organization_id)
            .map(|team| team.clone())
            .collect();
        Ok(filtered_teams)
    }

    fn remove(&self, id: Uuid) -> Result<Option<Team>, Box<dyn Error>> {
        let mut collection: MutexGuard<Vec<Team>> = COLLECTION.lock().unwrap();
        let found_team: Option<Team> = collection.iter().find(|team| team.id == Some(id)).cloned();
        if found_team.is_some() {
            collection.retain(|team| team.id != found_team.clone().unwrap().id)
        }
        Ok(found_team)
    }
}
