use super::p2p::Capability;

pub const SUPPORTED_BASED_CAPABILITIES: [Capability; 1] = [Capability::based(1)];
pub const PERIODIC_BLOCK_BROADCAST_INTERVAL: std::time::Duration =
    std::time::Duration::from_millis(500);
pub const PERIODIC_BATCH_BROADCAST_INTERVAL: std::time::Duration =
    std::time::Duration::from_millis(500);
pub mod l2_connection;
pub mod messages;

use crate::rlpx::BASED_CAPABILITY_OFFSET;
use crate::rlpx::connection::server::Established;
use crate::rlpx::error::RLPxError;
use crate::rlpx::l2::messages::{BatchSealed, NewBlock};
use crate::rlpx::message::RLPxMessage;
use crate::rlpx::protocol::Protocol;
use ethrex_rlp::error::RLPDecodeError;

/// Protocol implementation for the experimental `based` capability used by the
/// L2 components.  Like the other protocol stubs, it only exposes the
/// capability and does not yet provide decoding or handling logic.
pub struct BasedProtocol;

impl Protocol for BasedProtocol {
    fn capability(&self) -> Capability {
        Capability::based(1)
    }

    fn decode(&self, _id: u8, _bytes: &[u8]) -> Result<Box<dyn RLPxMessage>, RLPDecodeError> {
        Err(RLPDecodeError::MalformedData)
    }

    fn handle(
        &self,
        _state: &mut Established,
        _msg: Box<dyn RLPxMessage>,
    ) -> Result<(), RLPxError> {
        Ok(())
    }

    fn message_ids(&self) -> Vec<u8> {
        vec![
            BASED_CAPABILITY_OFFSET + NewBlock::CODE,
            BASED_CAPABILITY_OFFSET + BatchSealed::CODE,
        ]
    }
}
