use nom::{
    bytes::complete::tag, character::complete::alphanumeric1, multi::separated_list1, IResult,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("unknown decode error")]
    Unknown,
}

pub fn decode_domain_name(input: &str) -> Result<Vec<&str>, DecodeError> {
    let (_, parts) = decode_domain_name_inner(input).map_err(|_e| DecodeError::Unknown)?;
    Ok(parts)
}

fn decode_domain_name_inner(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(tag("."), alphanumeric1)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_domain_name_inner() {
        let domain_name = "example.com";
        let result = decode_domain_name_inner(domain_name);
        assert_eq!(result, Ok(("", vec!["example", "com"])));
    }
}
