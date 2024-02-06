use anyhow::Result;
use clap::{Parser, ValueEnum};
use dns_decode::{decode_domain_name, decode_message};
use dns_encode::encode_message;
use dns_types::*;
use std::{
    fmt::Display,
    net::{Ipv4Addr, UdpSocket},
};

#[derive(Parser)]
#[command(about, long_about = None)]
struct Cli {
    domain_name: String,
    #[arg(short, long, default_value_t = Ipv4Addr::from([1,1,1,1]))]
    server: Ipv4Addr,
    #[arg(short, long, default_value_t = ResourceType::A)]
    resource_type: ResourceType,
}

#[derive(ValueEnum, Clone, Debug)]
enum ResourceType {
    A,
    AAAA,
}

impl Display for ResourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceType::A => write!(f, "a"),
            ResourceType::AAAA => write!(f, "aaaa"),
        }
    }
}

impl Into<QueryType> for ResourceType {
    fn into(self) -> QueryType {
        match self {
            ResourceType::A => QueryType::A,
            ResourceType::AAAA => QueryType::AAAA,
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let socket = UdpSocket::bind("0.0.0.0:0")?;

    let mut send_buffer: Vec<u8> = Vec::with_capacity(100);
    let message = create_message(&cli.domain_name, cli.resource_type, rand::random())?;
    encode_message(&message, &mut send_buffer)?;

    socket.send_to(&send_buffer, (cli.server, 53))?;

    // Receives a single datagram message on the socket. If `buf` is too small to hold
    // the message, it will be cut off.
    let mut receive_buffer = [0u8; 300];
    let (received_size, _src) = socket.recv_from(&mut receive_buffer)?;
    if received_size == receive_buffer.len() {
        // TODO handle buffer better
        panic!("buffer is too small");
    }
    let decoded_response = decode_message(&receive_buffer[0..received_size]).unwrap();
    println!("{}", decoded_response);
    Ok(())
}

fn create_message<QT: Into<QueryType>>(
    domain_name: &str,
    query_type: QT,
    message_id: u16,
) -> Result<Message> {
    let query_name = decode_domain_name(&domain_name)?
        .iter()
        .map(|s| s.to_string())
        .collect();
    let message = Message {
        header: MessageHeader {
            message_id,
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
            name: query_name,
            query_type: query_type.into(),
            query_class: QueryClass::Internet,
        }],
        answers: vec![],
    };
    Ok(message)
}
