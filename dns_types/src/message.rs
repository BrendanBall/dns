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
        for a in self.answers.as_slice() {
            write!(f, "{}", a.rdata)?;
        }
        Ok(())
    }
}
