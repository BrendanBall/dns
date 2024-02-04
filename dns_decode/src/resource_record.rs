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

fn name(input: &[u8]) -> IResult<&[u8], Name> {
    // TODO implement pointer/label properly
    let (input, pointer) = be_u16(input)?;
    Ok((input, Name::Pointer(pointer & 0xc0ff)))
}

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
    // TODO fix
    println!("{:02x?}", &input.iter_elements().collect::<Vec<u8>>());
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

pub fn resource_record(input: &[u8]) -> IResult<&[u8], ResourceRecord> {
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
        let dns_resource_record_bytes = hex::decode("c00c0001000100005a0200045db8d822").unwrap();
        let result = resource_record(&dns_resource_record_bytes);

        assert_eq!(
            result,
            Ok((
                &b""[..],
                ResourceRecord {
                    name: Name::Pointer(49164),
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
        let dns_resource_record_bytes =
            hex::decode("c00c001c00010000eee6001026062800022000010248189325c81946").unwrap();
        let result = resource_record(&dns_resource_record_bytes);

        assert_eq!(
            result,
            Ok((
                &b""[..],
                ResourceRecord {
                    name: Name::Pointer(49164),
                    resource_type: ResourceType::AAAA,
                    resource_class: ResourceClass::Internet,
                    ttl: 61158,
                    rdata: ResourceData::AAAA(Ipv6Addr::from([
                        0x2606, 0x2800, 0x0220, 0x0001, 0x0248, 0x1893, 0x25c8, 0x1946
                    ]))
                }
            ))
        );
    }
}
