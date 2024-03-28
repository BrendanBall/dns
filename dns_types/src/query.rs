use std::{convert::Into, fmt::Display};

use crate::Name;

#[derive(Debug, PartialEq)]
pub struct Query {
    pub name: Name,
    pub query_type: QueryType,
    pub query_class: QueryClass,
}

impl Display for Query {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            ";; {}\t\t{}\t{}",
            self.name, self.query_class, self.query_type
        )?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum QueryType {
    A,
    NS,
    CNAME,
    SOA,
    WKS,
    PTR,
    MX,
    SRV,
    AAAA,
    ANY,
    Unknown(u16),
}

impl Display for QueryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            QueryType::A => write!(f, "A"),
            QueryType::NS => write!(f, "NS"),
            QueryType::CNAME => write!(f, "CNAME"),
            QueryType::SOA => write!(f, "SOA"),
            QueryType::WKS => write!(f, "WKS"),
            QueryType::PTR => write!(f, "PTR"),
            QueryType::MX => write!(f, "MX"),
            QueryType::SRV => write!(f, "SRV"),
            QueryType::AAAA => write!(f, "AAAA"),
            QueryType::ANY => write!(f, "ANY"),
            QueryType::Unknown(_) => write!(f, "Unknown"),
        }
    }
}

impl Into<QueryType> for u16 {
    fn into(self) -> QueryType {
        match self {
            1 => QueryType::A,
            2 => QueryType::NS,
            5 => QueryType::CNAME,
            6 => QueryType::SOA,
            11 => QueryType::WKS,
            12 => QueryType::PTR,
            15 => QueryType::MX,
            33 => QueryType::SRV,
            28 => QueryType::AAAA,
            255 => QueryType::ANY,
            u => QueryType::Unknown(u),
        }
    }
}

impl Into<u16> for QueryType {
    fn into(self) -> u16 {
        match self {
            QueryType::A => 1,
            QueryType::NS => 2,
            QueryType::CNAME => 5,
            QueryType::SOA => 6,
            QueryType::WKS => 11,
            QueryType::PTR => 12,
            QueryType::MX => 15,
            QueryType::SRV => 33,
            QueryType::AAAA => 28,
            QueryType::ANY => 255,
            QueryType::Unknown(u) => u,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum QueryClass {
    Internet,
    Unknown(u16),
}

impl Display for QueryClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            QueryClass::Internet => write!(f, "IN"),
            QueryClass::Unknown(i) => write!(f, "UNKNOWN({})", i),
        }
    }
}

impl Into<QueryClass> for u16 {
    fn into(self) -> QueryClass {
        match self {
            1 => QueryClass::Internet,
            u => QueryClass::Unknown(u),
        }
    }
}

impl Into<u16> for QueryClass {
    fn into(self) -> u16 {
        match self {
            QueryClass::Internet => 1,
            QueryClass::Unknown(u) => u,
        }
    }
}
