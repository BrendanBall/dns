use dns_types::query::*;

use nom::{
    bytes::complete::{tag, take},
    combinator::iterator,
    error::{Error, ErrorKind},
    number::complete::{be_u16, be_u8},
    Err, IResult,
};
use std::convert::Into;

use crate::input::DnsFrameInput;

fn label(input: DnsFrameInput) -> IResult<DnsFrameInput, String> {
    let (input, str_length) = be_u8(input)?;
    if str_length == 0 {
        return Err(Err::Error(Error::new(input, ErrorKind::Eof)));
    }
    // TODO validate max length
    let (input, str) = take(str_length)(input)?;
    Ok((input, String::from_utf8(str.input.to_vec()).unwrap()))
}

fn name(input: DnsFrameInput) -> IResult<DnsFrameInput, Vec<String>> {
    let mut it = iterator(input, label);
    let labels: Vec<String> = it.collect();
    let (input, ()) = it.finish()?;
    let (input, _) = tag(b"\x00")(input)?;
    Ok((input, labels))
}

pub fn query(input: DnsFrameInput) -> IResult<DnsFrameInput, Query> {
    let (input, name) = name(input)?;
    let (input, query_type) = be_u16(input)?;
    let (input, query_class) = be_u16(input)?;
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
        let result = query(DnsFrameInput::new(&dns_query_bytes));

        assert_eq!(
            result,
            Ok((
                DnsFrameInput {
                    frame: &dns_query_bytes,
                    input: &b""[..],
                },
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
        let result = name(DnsFrameInput::new(&dns_query_bytes));

        assert_eq!(
            result,
            Ok((
                DnsFrameInput {
                    frame: &dns_query_bytes,
                    input: &b""[..],
                },
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
        let result = label(DnsFrameInput::new(&dns_query_bytes));

        assert_eq!(
            result,
            Ok((
                DnsFrameInput {
                    frame: &dns_query_bytes,
                    input: &dns_query_bytes[4..],
                },
                String::from("www"),
            ))
        );

        let result = label(DnsFrameInput::new(&dns_query_bytes[4..]));
        assert_eq!(
            result,
            Ok((
                DnsFrameInput {
                    frame: &dns_query_bytes[4..],
                    input: &dns_query_bytes[13..],
                },
                String::from("mydomain"),
            ))
        );

        let result = label(DnsFrameInput::new(&dns_query_bytes[13..]));
        assert_eq!(
            result,
            Ok((
                DnsFrameInput {
                    frame: &dns_query_bytes[13..],
                    input: &[0u8][..]
                },
                String::from("com"),
            ))
        );
    }
}
