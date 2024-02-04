use nom::{
    bytes::complete::{tag, take},
    combinator::iterator,
    error::{Error, ErrorKind},
    number::complete::{be_u16, be_u8},
    Err, IResult,
};
use std::convert::Into;

#[derive(Debug, PartialEq)]
pub struct Query {
    pub name: Vec<String>,
    pub query_type: QueryType,
    pub query_class: QueryClass,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

fn label(input: &[u8]) -> IResult<&[u8], String> {
    let (input, str_length) = be_u8(input)?;
    if str_length == 0 {
        return Err(Err::Error(Error::new(input, ErrorKind::Eof)));
    }
    // TODO validate max length
    let (input, str) = take(str_length)(input)?;
    Ok((input, String::from_utf8(str.to_vec()).unwrap()))
}

fn name(input: &[u8]) -> IResult<&[u8], Vec<String>> {
    let mut it = iterator(input, label);
    let labels: Vec<String> = it.collect();
    let (input, ()) = it.finish()?;
    let (input, _) = tag(b"\x00")(input)?;
    Ok((input, labels))
}

pub fn query(input: &[u8]) -> IResult<&[u8], Query> {
    let (input, name) = name(input)?;
    let (input, query_type) = be_u16(input)?;
    let (input, query_class) = be_u16(input)?;
    println!("query class: {}", query_class);
    Ok((
        input,
        Query {
            name,
            query_type: query_type.into(),
            query_class: query_class.into(),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query() {
        let dns_query_bytes = hex::decode("076578616d706c6503636f6d0000010001").unwrap();
        let result = query(&dns_query_bytes);

        assert_eq!(
            result,
            Ok((
                &b""[..],
                Query {
                    name: vec![String::from("example"), String::from("com")],
                    query_type: QueryType::A,
                    query_class: QueryClass::Internet,
                }
            ))
        );
    }

    #[test]
    fn test_name() {
        let dns_query_bytes = hex::decode("03777777086D79646F6D61696E03636F6D00").unwrap();
        let result = name(&dns_query_bytes);

        assert_eq!(
            result,
            Ok((
                &b""[..],
                vec![
                    String::from("www"),
                    String::from("mydomain"),
                    String::from("com")
                ]
            ))
        );
    }

    #[test]
    fn test_label() {
        let dns_query_bytes = hex::decode("03777777086D79646F6D61696E03636F6D00").unwrap();
        let result = label(&dns_query_bytes);

        assert_eq!(result, Ok((&dns_query_bytes[4..], String::from("www"),)));

        let result = label(&dns_query_bytes[4..]);
        assert_eq!(
            result,
            Ok((&dns_query_bytes[13..], String::from("mydomain"),))
        );

        let result = label(&dns_query_bytes[13..]);
        assert_eq!(result, Ok((&[0u8][..], String::from("com"),)));
    }
}
