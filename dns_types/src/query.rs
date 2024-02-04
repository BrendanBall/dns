use std::convert::Into;

#[derive(Debug, PartialEq)]
pub struct Query {
    pub name: Vec<String>,
    pub query_type: QueryType,
    pub query_class: QueryClass,
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
