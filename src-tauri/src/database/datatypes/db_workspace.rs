use super::{db_project::DatabaseProject, db_wildcard::DatabaseWildcard};

/// The bottom-most part of the file-hierarchy
#[derive(Default)]
pub struct Workspace {
    id: u32,
    projects: Vec<DatabaseProject>,
    wildcards: Vec<DatabaseWildcard>
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
}