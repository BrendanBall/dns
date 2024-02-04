use dns_types::query::*;
use std::io::{Error, Write};

pub fn encode_query<W: Write>(query: &Query, writer: &mut W) -> Result<(), Error> {
    for label in query.name.as_slice() {
        let label_bytes = label.as_bytes();
        let label_len = [label_bytes.len() as u8];
        writer.write_all(&label_len)?;
        writer.write_all(label_bytes)?;
    }
    writer.write_all(b"\x00")?;
    let qtype: u16 = query.query_type.into();
    let qtype_bytes = qtype.to_be_bytes();
    writer.write_all(&qtype_bytes)?;
    let qclass: u16 = query.query_class.into();
    let qclass_bytes = qclass.to_be_bytes();
    writer.write_all(&qclass_bytes)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_query() {
        let query_bytes = hex::decode("076578616d706c6503636f6d0000010001").unwrap();
        let query = Query {
            name: vec![String::from("example"), String::from("com")],
            query_type: QueryType::A,
            query_class: QueryClass::Internet,
        };
        let mut buffer: Vec<u8> = Vec::with_capacity(12);
        encode_query(&query, &mut buffer).unwrap();
        assert_eq!(buffer, query_bytes);
    }
}
