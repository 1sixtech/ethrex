pub mod connection;
pub mod error;
pub mod eth;
pub mod initiator;
pub mod l2;
pub mod message;
pub mod p2p;
pub mod protocol;
pub mod protocol_registry;
pub mod snap;
pub mod utils;

pub const ETH_CAPABILITY_OFFSET: u8 = 0x10;
pub const SNAP_CAPABILITY_OFFSET: u8 = 0x21;
pub const BASED_CAPABILITY_OFFSET: u8 = 0x30;
pub const CUSTOM_CAPABILITY_OFFSET: u8 = 0x40;

pub use message::Message;
