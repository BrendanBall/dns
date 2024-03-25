use dns_types::resource_record::*;
use nom::{
    multi::length_value,
    number::complete::{be_u128, be_u16, be_u32},
    IResult, InputIter, InputLength, Slice,
};
use std::ops::RangeFrom;
use std::{
    convert::Into,
    net::{Ipv4Addr, Ipv6Addr},
};

use crate::{input::DnsFrameInput, name::name};

fn a_record<Input>(input: Input) -> IResult<Input, ResourceData>
where
    Input: Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength,
{
    let (input, ip_address) = be_u32(input)?;
    Ok((input, ResourceData::A(Ipv4Addr::from(ip_address))))
}

fn aaaa_record<Input>(input: Input) -> IResult<Input, ResourceData>
where
    Input: Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength,
{
    let (input, ip_address) = be_u128(input)?;
    Ok((input, ResourceData::AAAA(Ipv6Addr::from(ip_address))))
}

fn resource_data<Input>(
    resource_type: ResourceType,
) -> impl Fn(Input) -> IResult<Input, ResourceData>
where
    Input: Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength,
{
    move |input: Input| match resource_type {
        ResourceType::A => a_record(input),
        ResourceType::AAAA => aaaa_record(input),
        _ => todo!(),
    }
}

pub fn resource_record(input: DnsFrameInput) -> IResult<DnsFrameInput, ResourceRecord> {
    let (input, name) = name(input)?;
    let (input, resource_type) = be_u16(input)?;
    let (input, resource_class) = be_u16(input)?;
    let (input, ttl) = be_u32(input)?;
    let resource_type: ResourceType = resource_type.into();
    let (input, rdata) = length_value(be_u16, resource_data(resource_type))(input)?;
    Ok((
        input,
        ResourceRecord {
            name,
            resource_type,
            resource_class: resource_class.into(),
            ttl,
            rdata,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_record_a_record() {
        let message = hex::decode("690681800001000100000000076578616d706c6503636f6d0000010001c00c0001000100005a0200045db8d822").unwrap();
        let result = resource_record(DnsFrameInput {
            frame: &message,
            input: &message[29..],
        });

        assert_eq!(
            result,
            Ok((
                DnsFrameInput {
                    frame: &message,
                    input: &b""[..],
                },
                ResourceRecord {
                    name: vec![String::from("example"), String::from("com")],
                    resource_type: ResourceType::A,
                    resource_class: ResourceClass::Internet,
                    ttl: 23042,
                    rdata: ResourceData::A(Ipv4Addr::from(0x5db8d822))
                }
            ))
        );
    }

    #[test]
    fn test_resource_record_aaaa_record() {
        let message =
            hex::decode("653081800001000100000000076578616d706c6503636f6d00001c0001c00c001c0001000130c8001026062800022000010248189325c81946").unwrap();
        let result = resource_record(DnsFrameInput {
            frame: &message,
            input: &message[29..],
        });

        assert_eq!(
            result,
            Ok((
                DnsFrameInput {
                    frame: &message,
                    input: &b""[..],
                },
                ResourceRecord {
                    name: vec![String::from("example"), String::from("com")],
                    resource_type: ResourceType::AAAA,
                    resource_class: ResourceClass::Internet,
                    ttl: 78024,
                    rdata: ResourceData::AAAA(Ipv6Addr::from([
                        0x2606, 0x2800, 0x0220, 0x0001, 0x0248, 0x1893, 0x25c8, 0x1946
                    ]))
                }
            ))
        );
    }
}
