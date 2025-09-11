use crate::rlpx::connection::server::Established;
use crate::rlpx::message::RLPxMessage;
use crate::rlpx::p2p::Capability;
use crate::rlpx::error::RLPxError;
use ethrex_rlp::error::RLPDecodeError;

/// Trait implemented by rlpx subprotocols such as eth, snap or based.
///
/// A protocol exposes a capability which is advertised during the handshake.
/// It is also able to decode incoming messages that fall inside the range of
/// message identifiers associated with the protocol and to handle those
/// messages.
///
/// The default implementation in this repository is intentionally small.  It
/// only provides the hooks that are required by the `ProtocolRegistry` and the
/// `P2PContext::register_protocol` API so that external crates can plug their
/// own protocols in a flexible way.
pub trait Protocol: Send + Sync {
    /// Returns the capability that should be advertised during the hello
    /// handshake message.
    fn capability(&self) -> Capability;

    /// Decode the message identified by `id` using the provided `bytes`.
    /// Implementations should return a boxed `RLPxMessage` on success.
    fn decode(
        &self,
        id: u8,
        bytes: &[u8],
    ) -> Result<Box<dyn RLPxMessage>, RLPDecodeError>;

    /// Handle a previously decoded message.
    fn handle(
        &self,
        state: &mut Established,
        msg: Box<dyn RLPxMessage>,
    ) -> Result<(), RLPxError>;

    /// Returns the list of absolute message identifiers handled by this
    /// protocol.  The `ProtocolRegistry` uses this information to route
    /// messages to the proper handler.
    fn message_ids(&self) -> Vec<u8>;
}
