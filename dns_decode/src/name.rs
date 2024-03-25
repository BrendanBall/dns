use crate::input::DnsFrameInput;
use nom::{
    bytes::complete::{tag, take},
    combinator::iterator,
    error::{Error, ErrorKind},
    number::complete::{be_u16, be_u8},
    Err, IResult,
};

pub fn name(input: DnsFrameInput) -> IResult<DnsFrameInput, Vec<String>> {
    let mut it = iterator(input, name_label);
    let labels: Vec<String> = it.collect();
    let (input, ()) = it.finish()?;
    let (_, str_length) = be_u8(input)?;
    if is_pointer(str_length) {
        let (input, pointer_labels) = name_pointer(input)?;
        return Ok((input, [labels, pointer_labels].concat()));
    }
    let (input, _) = tag(b"\x00")(input)?;
    Ok((input, labels))
}

fn is_pointer(b: u8) -> bool {
    b & 0b11000000 == 0b11000000
}

fn label_size(b: u8) -> Option<u8> {
    if b == 0 || b & 0b11000000 != 0 {
        None
    } else {
        Some(b & 0b00111111)
    }
}

fn name_label(input: DnsFrameInput) -> IResult<DnsFrameInput, String> {
    let (input, str_length) = be_u8(input)?;
    match label_size(str_length) {
        None => Err(Err::Error(Error::new(input, ErrorKind::Eof))),
        Some(size) => {
            let (input, str) = take(size)(input)?;
            Ok((input, String::from_utf8(str.input.to_vec()).unwrap()))
        }
    }
}

fn name_pointer(input: DnsFrameInput) -> IResult<DnsFrameInput, Vec<String>> {
    let (input, pointer) = be_u16(input)?;
    let pointer = (pointer & 0b0011_1111_1111_1111) as usize;
    let (_, labels) = name(DnsFrameInput {
        frame: input.frame,
        input: &input.frame[pointer..],
    })?;
    Ok((input, labels))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name_label() {
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
    fn test_name_pointer() {
        let message = hex::decode("690681800001000100000000076578616d706c6503636f6d0000010001c00c0001000100005a0200045db8d822").unwrap();
        let result = name(DnsFrameInput {
            frame: &message,
            input: &message[29..],
        });

        assert_eq!(
            result,
            Ok((
                DnsFrameInput {
                    frame: &message,
                    input: &message[31..],
                },
                vec![String::from("example"), String::from("com")]
            ))
        );
    }

    #[test]
    fn test_label() {
        let dns_query_bytes = hex::decode("03777777086D79646F6D61696E03636F6D00").unwrap();
        let result = name_label(DnsFrameInput::new(&dns_query_bytes));

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

        let result = name_label(DnsFrameInput::new(&dns_query_bytes[4..]));
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

        let result = name_label(DnsFrameInput::new(&dns_query_bytes[13..]));
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
