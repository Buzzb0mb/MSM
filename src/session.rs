use crate::error::{MSMResult, MSMError};
use crate::session_parser;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::process::Command;
use std::fmt;

#[derive(Debug)]
pub struct Session {
    name: String,
    comment: Option<String>,
    exec: String
}

impl Session {
    pub fn execute(&self) -> MSMResult<()> {
        Command::new("startx").arg(&self.exec).output()?;
        Ok(())
    }
}

impl TryFrom<&[u8]> for Session {
    type Error = MSMError;
    fn try_from(data: &[u8]) -> MSMResult<Self> {
        let config: HashMap<&str, &str> = session_parser::session(data)?.1; // parse the data into a hashmap

        /* temporary function to get a value in the config hashmap and return a MissingSessionVariable
         * error if it doesn't exist */
        let get_conf_val = |c: &HashMap<&str, &str>, n: &'static str| {
            match c.get(n) {
                Some(v) => Ok(v.to_string()),
                None => Err(MSMError::MissingSessionVariable(n))
            }
        };

        let s = Session {
            name: get_conf_val(&config, "Name")?,
            comment: match config.get("Comment") {
                Some(c) => Some(c.to_string()),
                None => None
            },
            exec: get_conf_val(&config, "Exec")?
        };
        Ok(s)
    }
}

impl fmt::Display for Session {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(c) = &self.comment {
            write!(f, "{} - {}", self.name, c)
        } else {
            write!(f, "{}", self.name)
        }
    }
}

