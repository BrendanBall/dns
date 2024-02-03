use nom::{
    bits::{bits, streaming::take},
    error::Error,
    number::complete::be_u16,
    IResult,
};
use std::convert::Into;

#[derive(Debug, PartialEq)]
pub struct MessageHeader {
    pub message_id: u16,
    pub flags: Flags,
    pub query_count: u16,
    pub answer_count: u16,
    pub name_server_count: u16,
    pub additional_count: u16,
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

impl Into<Opcode> for u8 {
    fn into(self) -> Opcode {
        match self {
            0 => Opcode::Query,
            1 => Opcode::IQuery,
            2 => Opcode::Status,
            3 => Opcode::Unknown,
            _ => unreachable!(),
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

fn dns_flags(input: &[u8]) -> IResult<&[u8], Flags> {
    fn dns_flags_inner(
        input: (&[u8], usize),
    ) -> IResult<(&[u8], usize), Flags, Error<(&[u8], usize)>> {
        type BitParseResult<'a> = ((&'a [u8], usize), u8);
        let (input, qr): BitParseResult = take(1usize)(input)?;
        let (input, opcode): BitParseResult = take(4usize)(input)?;
        let (input, aa): BitParseResult = take(1usize)(input)?;
        let (input, truncated): BitParseResult = take(1usize)(input)?;
        let (input, recursion_desired): BitParseResult = take(1usize)(input)?;
        let (input, recursion_available): BitParseResult = take(1usize)(input)?;
        let (input, _reserved): BitParseResult = take(3usize)(input)?;
        let (input, rcode): BitParseResult = take(4usize)(input)?;
        Ok((
            input,
            Flags {
                qr: qr.into(),
                opcode: opcode.into(),
                aa: aa.into(),
                truncated: truncated.into(),
                recursion_desired: recursion_desired.into(),
                recursion_available: recursion_available.into(),
                rcode: rcode.into(),
            },
        ))
    }
    bits::<_, _, Error<(&[u8], usize)>, _, _>(dns_flags_inner)(input)
}

fn dns_message(input: &[u8]) -> IResult<&[u8], MessageHeader> {
    let (input, message_id) = be_u16(input)?;
    let (input, flags) = dns_flags(input)?;
    let (input, query_count) = be_u16(input)?;
    let (input, answer_count) = be_u16(input)?;
    let (input, name_server_count) = be_u16(input)?;
    let (input, additional_count) = be_u16(input)?;
    Ok((
        input,
        MessageHeader {
            message_id,
            flags,
            query_count,
            answer_count,
            name_server_count,
            additional_count,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_header_query() {
        let dns_message_bytes = hex::decode("690601000001000000000000").unwrap();

        let result = dns_message(&dns_message_bytes);
        assert_eq!(
            result,
            Ok((
                &b""[..],
                MessageHeader {
                    message_id: 0x6906,
                    flags: Flags {
                        qr: QR::Query,
                        opcode: Opcode::Query,
                        aa: AuthoritativeAnswer::NonAuthoritative,
                        truncated: Truncated::NotTruncated,
                        recursion_desired: RecursionDesired::Desired,
                        recursion_available: RecursionAvailable::NotAvailable,
                        rcode: Rcode::NoError,
                    },
                    query_count: 1,
                    answer_count: 0,
                    name_server_count: 0,
                    additional_count: 0,
                }
            ))
        );
    }

    #[test]
    fn message_header_response() {
        let dns_message_bytes = hex::decode("690681800001000100000000").unwrap();

        let result = dns_message(&dns_message_bytes);
        assert_eq!(
            result,
            Ok((
                &b""[..],
                MessageHeader {
                    message_id: 0x6906,
                    flags: Flags {
                        qr: QR::Response,
                        opcode: Opcode::Query,
                        aa: AuthoritativeAnswer::NonAuthoritative,
                        truncated: Truncated::NotTruncated,
                        recursion_desired: RecursionDesired::Desired,
                        recursion_available: RecursionAvailable::Available,
                        rcode: Rcode::NoError,
                    },
                    query_count: 1,
                    answer_count: 1,
                    name_server_count: 0,
                    additional_count: 0,
                }
            ))
        );
    }
}
