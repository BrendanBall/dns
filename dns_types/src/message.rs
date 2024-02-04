use crate::{message_header::*, query::*, resource_record::*};

#[derive(Debug, PartialEq)]
pub struct Message {
    pub header: MessageHeader,
    pub queries: Vec<Query>,
    pub answers: Vec<ResourceRecord>,
}
