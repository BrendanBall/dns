use bitvec::prelude::*;
use dns_types::message_header::*;
use std::io::{Error, Write};

fn encode_flags<W: Write>(flags: &Flags, writer: &mut W) -> Result<(), Error> {
    let mut flags_buffer = [0u8, 0u8];
    let qr: bool = match flags.qr {
        QR::Query => false,
        QR::Response => true,
    };
    let flags_bits = flags_buffer.view_bits_mut::<Msb0>();
    flags_bits.set(0, qr);
    // TODO opcode
    let rd: bool = match flags.recursion_desired {
        RecursionDesired::Desired => true,
        RecursionDesired::NotDesired => false,
    };
    flags_bits.set(7, rd);
    writer.write_all(&flags_buffer)?;
    Ok(())
}

pub fn encode_message_header<W: Write>(
    message_header: &MessageHeader,
    writer: &mut W,
) -> Result<(), Error> {
    let message_id_bytes = message_header.message_id.to_be_bytes();
    writer.write_all(&message_id_bytes)?;
    encode_flags(&message_header.flags, writer)?;
    let query_count_bytes = message_header.query_count.to_be_bytes();
    writer.write_all(&query_count_bytes)?;
    let answer_count_bytes = message_header.answer_count.to_be_bytes();
    writer.write_all(&answer_count_bytes)?;
    let name_server_count_bytes = message_header.name_server_count.to_be_bytes();
    writer.write_all(&name_server_count_bytes)?;
    let additional_count_bytes = message_header.additional_count.to_be_bytes();
    writer.write_all(&additional_count_bytes)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_message_header() {
        let message_header_bytes = hex::decode("690601000001000000000000").unwrap();
        let message_header = MessageHeader {
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
        };
        let mut buffer: Vec<u8> = Vec::with_capacity(12);
        encode_message_header(&message_header, &mut buffer).unwrap();
        assert_eq!(buffer, message_header_bytes);
    }
}
