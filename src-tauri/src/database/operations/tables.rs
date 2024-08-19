#[derive(PartialEq, Eq, Hash, Clone)]
pub enum DatabaseTable {
    Subjects,
    Wildcards,
    AppSettings,
    TrackedFiles,
    Workspace,
}

impl DatabaseTable {
    pub fn to_str(&self) -> String {
        match *self {
            DatabaseTable::Subjects => "Subjects".to_owned(),
            DatabaseTable::Wildcards => "Wildcards".to_owned(),
            DatabaseTable::AppSettings => "AppSettings".to_owned(),
            DatabaseTable::TrackedFiles => "TrackedFiles".to_owned(),
            DatabaseTable::Workspace => "Workspace".to_owned(),
        }
    }
}
