pub enum DatabaseTable{
    Projects,
    Wildcards,
    AppSettings
}

impl DatabaseTable {
    pub fn to_str(&self) -> String {
        match *self {
            DatabaseTable::Projects => "Projects".to_owned(),
            DatabaseTable::Wildcards => "Wildcards".to_owned(),
            DatabaseTable::AppSettings => "AppSettings".to_owned(),
        }
    }
}