use std::{
    convert::Into,
    fmt::Display,
    net::{Ipv4Addr, Ipv6Addr},
};

use crate::Name;

#[derive(Debug, PartialEq)]
pub struct ResourceRecord {
    pub name: Name,
    pub resource_type: ResourceType,
    pub resource_class: ResourceClass,
    pub ttl: u32,
    pub rdata: ResourceData,
}

impl Display for ResourceRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}\t\t{}\t{}\t{}\t{}",
            self.name, self.ttl, self.resource_class, self.resource_type, self.rdata
        )?;
        Ok(())
    }
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

impl Display for ResourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ResourceType::A => write!(f, "A"),
            ResourceType::NS => write!(f, "NS"),
            ResourceType::CNAME => write!(f, "CNAME"),
            ResourceType::SOA => write!(f, "SOA"),
            ResourceType::WKS => write!(f, "WKS"),
            ResourceType::PTR => write!(f, "PTR"),
            ResourceType::MX => write!(f, "MX"),
            ResourceType::SRV => write!(f, "SRV"),
            ResourceType::AAAA => write!(f, "AAAA"),
            ResourceType::Unknown(_) => write!(f, "Unknown"),
        }
    }
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

impl Display for ResourceClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ResourceClass::Internet => write!(f, "IN"),
            ResourceClass::Unknown(i) => write!(f, "UNKNOWN({})", i),
        }
    }
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

impl Display for ResourceData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceData::A(ip) => write!(f, "{}", ip),
            ResourceData::AAAA(ip) => write!(f, "{}", ip),
            _ => todo!(),
        }
    }
}
