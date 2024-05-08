use rusqlite::Statement;

pub trait DatabaseItem : Default {
    type Item;

    fn parse(&self, stmt: &mut Statement) -> Option<Self>;
    fn fields(&self) -> String;
}