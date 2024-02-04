use dns_types::message_header::*;

use nom::{
    bits::{bits, streaming::take},
    error::Error,
    number::complete::be_u16,
    IResult,
};
use std::convert::Into;

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

pub fn message_header(input: &[u8]) -> IResult<&[u8], MessageHeader> {
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

        let result = message_header(&dns_message_bytes);
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

        let result = message_header(&dns_message_bytes);
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
