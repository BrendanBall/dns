use dns_types::query::*;

use nom::{number::complete::be_u16, IResult};
use std::convert::Into;

use crate::{input::DnsFrameInput, name::name};

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
}
