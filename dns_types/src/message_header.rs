use std::{convert::Into, fmt::Display};

#[derive(Debug, PartialEq)]
pub struct MessageHeader {
    pub message_id: u16,
    pub flags: Flags,
    pub query_count: u16,
    pub answer_count: u16,
    pub name_server_count: u16,
    pub additional_count: u16,
}

impl Display for MessageHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            ";; ->>HEADER<<- opcode: {}, rcode: {}, id: {}",
            self.flags.opcode, self.flags.rcode, self.message_id
        )?;
        writeln!(
            f,
            ";; flags:{} ; QUERY: {}, ANSWER: {}, AUTHORITY: {}, ADDITIONAL: {}",
            self.flags,
            self.query_count,
            self.answer_count,
            self.name_server_count,
            self.additional_count,
        )?;
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub struct Flags {
    pub qr: QR,
    pub opcode: Opcode,
    pub aa: AuthoritativeAnswer,
    pub truncated: Truncated,
    pub recursion_desired: RecursionDesired,
    pub recursion_available: RecursionAvailable,
    pub rcode: Rcode,
}

impl Display for Flags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.qr == QR::Response {
            write!(f, " qr")?;
        }
        if self.aa == AuthoritativeAnswer::Authoritative {
            write!(f, " aa")?;
        }
        if self.truncated == Truncated::Truncated {
            write!(f, " tc")?;
        }
        if self.recursion_desired == RecursionDesired::Desired {
            write!(f, " rd")?;
        }
        if self.recursion_available == RecursionAvailable::Available {
            write!(f, " ra")?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum QR {
    Query,
    Response,
}

impl Into<QR> for u8 {
    fn into(self) -> QR {
        match self {
            0 => QR::Query,
            1 => QR::Response,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Opcode {
    Query,
    IQuery,
    Status,
    Unknown,
}

impl Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Opcode::Query => "QUERY",
            Opcode::IQuery => "IQUERY",
            Opcode::Status => "STATUS",
            Opcode::Unknown => "UNKNOWN",
        };
        write!(f, "{}", s)?;
        Ok(())
    }
}

impl Into<Opcode> for u8 {
    fn into(self) -> Opcode {
        match self {
            0 => Opcode::Query,
            1 => Opcode::IQuery,
            2 => Opcode::Status,
            3 => Opcode::Unknown,
            opcode => unreachable!("unexpected opcode: {}", opcode),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum AuthoritativeAnswer {
    Authoritative,
    NonAuthoritative,
}

impl Into<AuthoritativeAnswer> for u8 {
    fn into(self) -> AuthoritativeAnswer {
        match self {
            0 => AuthoritativeAnswer::NonAuthoritative,
            1 => AuthoritativeAnswer::Authoritative,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Truncated {
    Truncated,
    NotTruncated,
}

impl Into<Truncated> for u8 {
    fn into(self) -> Truncated {
        match self {
            0 => Truncated::NotTruncated,
            1 => Truncated::Truncated,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RecursionDesired {
    Desired,
    NotDesired,
}

impl Into<RecursionDesired> for u8 {
    fn into(self) -> RecursionDesired {
        match self {
            0 => RecursionDesired::NotDesired,
            1 => RecursionDesired::Desired,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RecursionAvailable {
    Available,
    NotAvailable,
}

impl Into<RecursionAvailable> for u8 {
    fn into(self) -> RecursionAvailable {
        match self {
            0 => RecursionAvailable::NotAvailable,
            1 => RecursionAvailable::Available,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Rcode {
    NoError,
    FormatError,
    ServerFailure,
    NameError,
    NotImplemented,
    Refused,
}

impl Display for Rcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Rcode::NoError => "NOERROR",
            Rcode::FormatError => "FORMATERROR",
            Rcode::ServerFailure => "SERVERFAILURE",
            Rcode::NameError => "NAMEERROR",
            Rcode::NotImplemented => "NOTIMPLEMENTED",
            Rcode::Refused => "REFUSED",
        };
        write!(f, "{}", s)?;
        Ok(())
    }
}

impl Into<Rcode> for u8 {
    fn into(self) -> Rcode {
        match self {
            0 => Rcode::NoError,
            1 => Rcode::FormatError,
            2 => Rcode::ServerFailure,
            3 => Rcode::NameError,
            4 => Rcode::NotImplemented,
            5 => Rcode::Refused,
            _ => unreachable!(),
        }
    }
}
