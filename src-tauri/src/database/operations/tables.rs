#[derive(PartialEq, Eq, Hash, Clone)]
pub enum DatabaseTable{
    Projects,
    Wildcards,
    AppSettings,
    TrackedFiles
}

impl DatabaseTable {
    pub fn to_str(&self) -> String {
        match *self {
            DatabaseTable::Projects => "Projects".to_owned(),
            DatabaseTable::Wildcards => "Wildcards".to_owned(),
            DatabaseTable::AppSettings => "AppSettings".to_owned(),
            DatabaseTable::TrackedFiles => "TrackedFiles".to_owned()
        }
    }
}