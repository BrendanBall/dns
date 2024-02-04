use crate::message_header::encode_message_header;
use crate::query::encode_query;
use dns_types::*;
use std::io::{Error, Write};

pub fn encode_message<W: Write>(message: &Message, writer: &mut W) -> Result<(), Error> {
    encode_message_header(&message.header, writer)?;
    for query in message.queries.as_slice() {
        encode_query(query, writer)?;
    }
    // TODO answers
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_message() {
        let message_bytes =
            hex::decode("690601000001000000000000076578616d706c6503636f6d0000010001").unwrap();
        let message = Message {
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
            answers: vec![],
        };
        let mut buffer: Vec<u8> = Vec::with_capacity(50);
        encode_message(&message, &mut buffer).unwrap();
        assert_eq!(buffer, message_bytes);
    }
}
