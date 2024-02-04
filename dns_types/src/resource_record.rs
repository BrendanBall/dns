use std::{
    convert::Into,
    net::{Ipv4Addr, Ipv6Addr},
};

#[derive(Debug, PartialEq)]
pub struct ResourceRecord {
    pub name: Name,
    pub resource_type: ResourceType,
    pub resource_class: ResourceClass,
    pub ttl: u32,
    pub rdata: ResourceData,
}

#[derive(Debug, PartialEq)]
pub enum Name {
    Value(Vec<String>),
    Pointer(u16),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ResourceType {
    A,
    NS,
    CNAME,
    SOA,
    WKS,
    PTR,
    MX,
    SRV,
    AAAA,
    Unknown(u16),
}

impl Into<ResourceType> for u16 {
    fn into(self) -> ResourceType {
        match self {
            1 => ResourceType::A,
            2 => ResourceType::NS,
            5 => ResourceType::CNAME,
            6 => ResourceType::SOA,
            11 => ResourceType::WKS,
            12 => ResourceType::PTR,
            15 => ResourceType::MX,
            33 => ResourceType::SRV,
            28 => ResourceType::AAAA,
            u => ResourceType::Unknown(u),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ResourceClass {
    Internet,
    Unknown(u16),
}

impl Into<ResourceClass> for u16 {
    fn into(self) -> ResourceClass {
        match self {
            1 => ResourceClass::Internet,
            u => ResourceClass::Unknown(u),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ResourceData {
    SOA,
    MX,
    A(Ipv4Addr),
    AAAA(Ipv6Addr),
    PTR,
    NS,
}
