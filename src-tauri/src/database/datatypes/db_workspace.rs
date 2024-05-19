use rusqlite::types::Value;
use tauri::AppHandle;

use crate::{database::operations::{db_item::DatabaseItem, tables::DatabaseTable}, state::ServiceAccess};

use super::{db_project::DatabaseProject, db_wildcard::DatabaseWildcard};

/// The bottom-most part of the file-hierarchy
#[derive(Default)]
pub struct Workspace {
    id: u32,
    wildcard_ids: Vec<u32>,
    project_ids: Vec<u32>,
    wildcards: Vec<DatabaseWildcard>,
    projects: Vec<DatabaseProject>
}

impl PartialEq for Workspace {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Workspace {
    pub fn add_project(&mut self, project: &DatabaseProject) {
        if self.projects.contains(project) { return; }
        self.projects.push(project.clone());
    }

    pub fn add_wildcard(&mut self, wildcard: &DatabaseWildcard) {
        if self.wildcards.contains(wildcard) { return; }
        self.wildcards.push(wildcard.clone())
    }

    pub fn wildcards(&self) -> &Vec<DatabaseWildcard> {
        &self.wildcards
    }

    pub fn projetcs(&self) -> &Vec<DatabaseProject> {
        &self.projects
    }

    pub fn load(&mut self, handle: &AppHandle, load_children: bool){
        self.load_wildcards_internal(handle);
        self.load_projects_internal(handle, load_children);
    }

    fn load_wildcards_internal(&mut self, handle: &AppHandle) {
        self.wildcards = self.wildcard_ids.iter().map(|w| DatabaseWildcard::from_id(w).read(handle).unwrap()).collect();
    }

    fn load_projects_internal(&mut self, handle: &AppHandle, load_children: bool) {
        let mut projects: Vec<DatabaseProject> = self.project_ids.iter().map(|p| DatabaseProject::from_id(p).read(handle).unwrap()).collect();
        if load_children {
            projects.iter_mut().for_each(|x| x.load(handle, true));
        }
        self.projects = projects;
    }

    pub fn from_id(id: &u32) -> Workspace {
        Workspace {
            id: *id,
            ..Default::default()
        }
    }

    pub fn from_project(handle: &AppHandle, project: &DatabaseProject) -> Workspace {
        let unique_id = handle.db_session(|session| session.get_and_claim_id(DatabaseTable::Workspace));
        let wildcard_ids = project.wildcards().iter().map(|w| w.id).collect();
        let project_ids = project.projects().iter().map(|p| p.id).collect();
        Workspace {
            id: unique_id.unwrap(),
            wildcard_ids: wildcard_ids,
            project_ids,
            wildcards: project.wildcards().clone(),
            projects: project.projects().clone(),
        }
    }
}

impl DatabaseItem for Workspace {
    type Item = Workspace;

    fn parse(&self, stmt: &mut rusqlite::Statement) -> Result<Self, rusqlite::Error> {
        let data = stmt.query_row([], |row| {
            let wcs = row.get::<usize, String>(1).expect("Should be able to deserialize wildcards");
            let projects = row.get::<usize, String>(2).expect("Should be able to deserialize projects");
            Ok(Workspace{
                id: row.get(0)?,
                wildcard_ids: serde_json::from_str(&wcs).unwrap(),
                project_ids: serde_json::from_str(&projects).unwrap(),
                ..Default::default()
            })
        });

        match data {
            Ok(x) => Ok(x),
            Err(e) => Err(e),
        }
    }

    fn id(&self) -> u32 {
        self.id
    }

    fn table(&self) -> DatabaseTable {
        DatabaseTable::Workspace
    }

    fn fields<'a>(&self) -> Vec<String> {
        vec!["id", "wildcards", "projects"].iter().map(|f| String::from(*f)).collect()
    }

    fn values<'a>(&self) -> Vec<rusqlite::types::Value> {
        let mut values: Vec<Value> = Vec::new();
        let wildcard_ids: Vec<u32> = self.wildcards.iter().map(|w| w.id).collect();
        let project_ids: Vec<u32> = self.projects.iter().map(|p| p.id).collect();

        values.push(self.id.into());
        values.push(serde_json::to_string(&wildcard_ids).expect("Should be able to serialize JSON").into());
        values.push(serde_json::to_string(&project_ids).expect("Should be able to serialize JSON").into());

        values
    }
}

impl serde::Serialize for Workspace {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            use serde::ser::SerializeStruct;

            let mut state = serializer.serialize_struct("Workspace", 4)?;
            state.serialize_field("id", &self.id)?;
            state.serialize_field("name", "wildcards")?;
            state.serialize_field("wildcards", &self.wildcards)?;
            state.serialize_field("projects", &self.projects)?;
            state.end()
    }
}