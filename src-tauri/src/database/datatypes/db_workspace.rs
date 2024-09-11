use std::path::Path;

use rusqlite::types::Value;
use tauri::AppHandle;
use uuid::Uuid;

use crate::{
    database::operations::{db_item::DatabaseItem, tables::DatabaseTable}, deployment::{deployable::Deployable, deployment::Deployment}, logging::logger::LogVisibility, state::ServiceAccess
};

use super::{db_subject::DatabaseSubject, db_wildcard::DatabaseWildcard};

/// The bottom-most part of the file-hierarchy
#[derive(Default)]
pub struct Workspace {
    id: String,
    wildcard_ids: Vec<String>,
    subject_ids: Vec<String>,
    wildcards: Vec<DatabaseWildcard>,
    subjects: Vec<DatabaseSubject>,
}

impl PartialEq for Workspace {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Workspace {
    pub fn add_project(&mut self, subject: &DatabaseSubject) {
        if self.subjects.contains(subject) {
            return;
        }
        self.subjects.push(subject.clone());
    }

    pub fn add_wildcard(&mut self, wildcard: &DatabaseWildcard) {
        if self.wildcards.contains(wildcard) {
            return;
        }
        self.wildcards.push(wildcard.clone())
    }

    pub fn wildcards(&self) -> &Vec<DatabaseWildcard> {
        &self.wildcards
    }

    pub fn projetcs(&self) -> &Vec<DatabaseSubject> {
        &self.subjects
    }

    pub fn load(&mut self, handle: &AppHandle, load_children: bool) {
        handle.logger(|lgr| lgr.log_trace("Loading workspace", "WorkspaceLoad", LogVisibility::Backend));
        self.load_wildcards_internal(handle);
        self.load_subjects_internal(handle, load_children);
    }

    pub fn generate_deployment(&mut self,base_path: impl AsRef<Path>, handle: &AppHandle) -> Deployment {
        handle.logger(|lgr| lgr.log("Generating workspace deployment...", "WorkspaceDeployGen", LogVisibility::Backend, "DEPLOY"));
        let mut deployment: Deployment = Deployment::new(&base_path);
        
        for subject in &mut self.subjects {
            subject.load_merge_definitions(handle);
            let node = subject.generate_deploy_node(&base_path, handle);

            match node {
                Some(x) => deployment.add_node(x),
                None => (),
            }
        }

        for wildcard in &self.wildcards {
            let node = wildcard.generate_deploy_node(&base_path, handle);

            match node {
                Some(x) => deployment.add_node(x),
                None => (),
            }
        }

        deployment
    }

    fn load_wildcards_internal(&mut self, handle: &AppHandle) {
        handle.logger(|lgr| lgr.log_trace("Loading workspace wildcards", "WorkspaceLoad_Wildcards", LogVisibility::Backend));
        self.wildcards = self
            .wildcard_ids
            .iter()
            .map(|w| DatabaseWildcard::from_id(w).read_db(handle).unwrap())
            .collect();
    }

    fn load_subjects_internal(&mut self, handle: &AppHandle, load_children: bool) {
        handle.logger(|lgr| lgr.log_trace("Loading workspace subjects", "WorkspaceLoad_Subjects", LogVisibility::Backend));
        let mut subjects: Vec<DatabaseSubject> = self
            .subject_ids
            .iter()
            .map(|p| DatabaseSubject::from_id(p).read_db(handle).unwrap())
            .collect();
        if load_children {
            subjects.iter_mut().for_each(|x| x.load(handle, true));
        }
        self.subjects = subjects;
    }

    pub fn from_id(id: &str) -> Workspace {
        Workspace {
            id: id.to_owned(),
            ..Default::default()
        }
    }

    pub fn contains(&self, uuid: &String) -> bool {
        self.wildcard_ids.contains(uuid) || self.subject_ids.contains(uuid)
    }

    pub fn from_subject(handle: &AppHandle, subject: &DatabaseSubject) -> Workspace {
        let unique_id = Uuid::nil();
        let wildcard_ids = subject.wildcards().iter().map(|w| w.uuid.clone()).collect();
        let subject_ids = subject.subjects().iter().map(|p| p.uuid.clone()).collect();
        Workspace {
            id: unique_id.to_string(),
            wildcard_ids: wildcard_ids,
            subject_ids,
            wildcards: subject.wildcards().clone(),
            subjects: subject.subjects().clone(),
        }
    }
}

impl DatabaseItem for Workspace {
    type Item = Workspace;

    fn parse(&self, stmt: &mut rusqlite::Statement) -> Result<Self, rusqlite::Error> {
        let data = stmt.query_row([], |row| {
            let wcs = row
                .get::<usize, String>(1)
                .expect("Should be able to deserialize wildcards");
            let subjects = row
                .get::<usize, String>(2)
                .expect("Should be able to deserialize subjects");
            Ok(Workspace {
                id: row.get(0)?,
                wildcard_ids: serde_json::from_str(&wcs).unwrap(),
                subject_ids: serde_json::from_str(&subjects).unwrap(),
                ..Default::default()
            })
        });

        match data {
            Ok(x) => Ok(x),
            Err(e) => Err(e),
        }
    }

    fn id(&self) -> String {
        self.id.clone()
    }

    fn table(&self) -> DatabaseTable {
        DatabaseTable::Workspace
    }

    fn fields<'a>(&self) -> Vec<String> {
        vec!["uuid", "wildcards", "subjects"]
            .iter()
            .map(|f| String::from(*f))
            .collect()
    }

    fn values<'a>(&self) -> Vec<rusqlite::types::Value> {
        let mut values: Vec<Value> = Vec::new();
        let wildcard_ids: Vec<String> = self.wildcards.iter().map(|w| w.uuid.clone()).collect();
        let subject_ids: Vec<String> = self.subjects.iter().map(|p| p.uuid.clone()).collect();

        values.push(self.id.clone().into());
        values.push(
            serde_json::to_string(&wildcard_ids)
                .expect("Should be able to serialize JSON")
                .into(),
        );
        values.push(
            serde_json::to_string(&subject_ids)
                .expect("Should be able to serialize JSON")
                .into(),
        );

        values
    }
}

impl serde::Serialize for Workspace {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("Workspace", 4)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("name", "wildcards")?;
        state.serialize_field("wildcards", &self.wildcards)?;
        state.serialize_field("subjects", &self.subjects)?;
        state.end()
    }
}
