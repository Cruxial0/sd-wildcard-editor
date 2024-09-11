use std::io::Error;

pub trait WildcardSyntaxExt {
    fn from_wildcard(&self) -> Result<String, String>;
    fn to_wildcard(&self) -> String;
}

impl WildcardSyntaxExt for String {
    fn from_wildcard(&self) -> Result<String, String> {
        if self.starts_with("__") && self.ends_with("__") {
            let text: &str = &self;
            Ok(text[2..text.len()-2].to_owned())
        } else {
            Err(String::from("Input string is not a valid wildcard format."))
        }
    }

    fn to_wildcard(&self) -> String {
        format!("__{}__", self)
    }
}