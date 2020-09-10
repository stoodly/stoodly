use std::error::Error;

use mongodb::bson::{doc, Bson, Document};
use mongodb::{options::UpdateOptions, sync::Collection};
use uuid::Uuid;
use domain::organization::team::{Repository, Team};

pub struct TeamRepository {
    pub collection: Collection,
}

impl Repository for TeamRepository {
    fn add(&self, team: Team) -> Result<Team, Box<dyn Error>> {
        let mut mut_team: Team = team.clone();
        let id: String = mut_team.id.get_or_insert(Uuid::new_v4()).to_string();
        let member_ids: Vec<String> = mut_team.members.iter().map(|id| id.to_string()).collect();
        self.collection.update_one(
            doc! {
                "_id": &id
            },
            doc! {
                "_id": &id,
                "organization_id": &mut_team.organization_id.to_string(),
                "name": &mut_team.name,
                "members": &member_ids,
            },
            Some(UpdateOptions::builder().upsert(Some(true)).build()),
        )?;
        Ok(mut_team)
    }

    fn find_by_id(&self, id: Uuid) -> Result<Option<Team>, Box<dyn Error>> {
        let document_opt: Option<Document> = self
            .collection
            .find_one(doc! { "_id": id.to_string() }, None)?;
        match document_opt {
            Some(document) => Ok(Some(document_to_team(document)?)),
            None => Ok(None),
        }
    }

    fn find_all_by_organization_id(
        &self,
        organization_id: Uuid,
    ) -> Result<Vec<Team>, Box<dyn Error>> {
        Ok(self
            .collection
            .find(
                doc! { "organization_id": organization_id.to_string() },
                None,
            )?
            .into_iter()
            .filter_map(Result::ok)
            .map(document_to_team)
            .filter_map(Result::ok)
            .collect())
    }

    fn remove(&self, id: Uuid) -> Result<Option<Team>, Box<dyn Error>> {
        let team: Result<Option<Team>, Box<dyn Error>> = self.find_by_id(id);
        self.collection.delete_one(
            doc! {
                "_id": &id.to_string()
            },
            None,
        )?;
        team
    }
}

fn document_to_team(document: Document) -> Result<Team, Box<dyn Error>> {
    let id: &str = document
        .get("_id")
        .and_then(Bson::as_str)
        .ok_or("missing _id")?;
    let organization_id: &str = document
        .get("organization_id")
        .and_then(Bson::as_str)
        .ok_or("missing organization_id")?;
    let name: String = document
        .get("name")
        .and_then(Bson::as_str)
        .map(|name| name.to_string())
        .ok_or("missing name")?;
    let members: Vec<Uuid> = document
        .get("members")
        .and_then(Bson::as_array)
        .map(|bsons| bsons.iter().map(|bson| bson.as_str()))
        .ok_or("missing members")?
        .flatten()
        .map(|id| Uuid::parse_str(id))
        .filter_map(Result::ok)
        .collect();
    let team: Team = Team {
        id: Some(Uuid::parse_str(id)?),
        organization_id: Uuid::parse_str(organization_id)?,
        name,
        members,
    };
    Ok(team)
}
