use std::fmt::Display;

use crate::{message_header::*, query::*, resource_record::*};

#[derive(Debug, PartialEq)]
pub struct Message {
    pub header: MessageHeader,
    pub queries: Vec<Query>,
    pub answers: Vec<ResourceRecord>,
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.header)?;
        writeln!(f, ";; QUESTION SECTION:")?;
        for query in self.queries.as_slice() {
            write!(f, "{}", query)?;
        }
        writeln!(f, "")?;
        writeln!(f, ";; ANSWER SECTION:")?;
        for answer in self.answers.as_slice() {
            write!(f, "{}", answer)?;
        }
        Ok(())
    }
}
