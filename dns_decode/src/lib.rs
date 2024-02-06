pub mod domain_name;
pub mod message;
pub mod message_header;
pub mod query;
pub mod resource_record;
pub use domain_name::decode_domain_name;
pub use message::decode_message;
