use crate::{message_header::*, query::*, resource_record::*};
use dns_types::*;
use nom::{multi::count, IResult};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("unknown parse error")]
    Unknown,
}

pub fn decode_message(input: &[u8]) -> Result<Message, DecodeError> {
    // TODO improve error reporting
    let (_, m) = message(input).map_err(|_op| DecodeError::Unknown)?;
    Ok(m)
}

fn message(input: &[u8]) -> IResult<&[u8], Message> {
    let (input, header) = message_header(input)?;
    let (input, queries) = count(query, header.query_count as usize)(input)?;
    let (input, answers) = count(resource_record, header.answer_count as usize)(input)?;

    Ok((
        input,
        Message {
            header,
            queries,
            answers,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn test_message_request() {
        let message_bytes =
            hex::decode("690601000001000000000000076578616d706c6503636f6d0000010001").unwrap();
        let result = message(&message_bytes);

        assert_eq!(
            result,
            Ok((
                &b""[..],
                Message {
                    header: MessageHeader {
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
                    },
                    queries: vec![Query {
                        name: vec![String::from("example"), String::from("com")],
                        query_type: QueryType::A,
                        query_class: QueryClass::Internet,
                    }],
                    answers: vec![]
                }
            ))
        );
    }

    #[test]
    fn test_message_response() {
        let message_bytes = hex::decode(
            "690681800001000100000000076578616d706c6503636f6d0000010001c00c0001000100005a0200045db8d822"
        )
        .unwrap();
        let result = message(&message_bytes);

        assert_eq!(
            result,
            Ok((
                &b""[..],
                Message {
                    header: MessageHeader {
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
                    },
                    queries: vec![Query {
                        name: vec![String::from("example"), String::from("com")],
                        query_type: QueryType::A,
                        query_class: QueryClass::Internet,
                    }],
                    answers: vec![ResourceRecord {
                        name: Name::Pointer(49164),
                        resource_type: ResourceType::A,
                        resource_class: ResourceClass::Internet,
                        ttl: 23042,
                        rdata: ResourceData::A(Ipv4Addr::from(0x5db8d822))
                    }]
                }
            ))
        );
    }
}
