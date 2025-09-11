pub(crate) mod backend;
pub(crate) mod blocks;
mod eth68;
mod eth69;
pub(crate) mod receipts;
pub(crate) mod status;
pub(crate) mod transactions;
pub(crate) mod update;

use crate::rlpx::ETH_CAPABILITY_OFFSET;
use crate::rlpx::connection::server::Established;
use crate::rlpx::error::RLPxError;
use crate::rlpx::eth::blocks::{BlockBodies, BlockHeaders, GetBlockBodies, GetBlockHeaders};
use crate::rlpx::eth::receipts::{GetReceipts, Receipts};
use crate::rlpx::eth::status::StatusMessage;
use crate::rlpx::eth::transactions::{
    GetPooledTransactions, NewPooledTransactionHashes, PooledTransactions, Transactions,
};
use crate::rlpx::message::RLPxMessage;
use crate::rlpx::p2p::Capability;
use crate::rlpx::protocol::Protocol;
use ethrex_rlp::error::RLPDecodeError;

/// Minimal implementation of the [`Protocol`] trait for the `eth` capability.
///
/// The handler methods are intentionally left empty; the current networking
/// stack still uses the legacy dispatch mechanism.  The implementation exists
/// so external crates can register the capability through the new
/// `ProtocolRegistry` API.
pub struct EthProtocol;

impl Protocol for EthProtocol {
    fn capability(&self) -> Capability {
        Capability::eth(68)
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
            ETH_CAPABILITY_OFFSET + StatusMessage::CODE,
            ETH_CAPABILITY_OFFSET + Transactions::CODE,
            ETH_CAPABILITY_OFFSET + GetBlockHeaders::CODE,
            ETH_CAPABILITY_OFFSET + BlockHeaders::CODE,
            ETH_CAPABILITY_OFFSET + GetBlockBodies::CODE,
            ETH_CAPABILITY_OFFSET + BlockBodies::CODE,
            ETH_CAPABILITY_OFFSET + NewPooledTransactionHashes::CODE,
            ETH_CAPABILITY_OFFSET + GetPooledTransactions::CODE,
            ETH_CAPABILITY_OFFSET + PooledTransactions::CODE,
            ETH_CAPABILITY_OFFSET + GetReceipts::CODE,
            ETH_CAPABILITY_OFFSET + Receipts::CODE,
        ]
    }
}
