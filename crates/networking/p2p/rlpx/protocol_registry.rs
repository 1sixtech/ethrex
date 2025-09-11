use std::collections::HashMap;
use std::sync::Arc;

use super::p2p::Capability;
use super::protocol::Protocol;

/// Registry that keeps track of the available subprotocols.  It maps message
/// identifiers to a protocol implementation which in turn is responsible for
/// decoding and handling the message.
#[derive(Default, Clone)]
pub struct ProtocolRegistry {
    /// Registered protocols keyed by the message identifier they handle.
    handlers: HashMap<u8, Arc<dyn Protocol>>,
    /// All registered protocols, even those without message identifiers. This is
    /// used to advertise their capabilities during the handshake.
    protocols: Vec<Arc<dyn Protocol>>,
}

impl std::fmt::Debug for ProtocolRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProtocolRegistry")
            .field("handlers", &self.handlers.keys().collect::<Vec<_>>())
            .field(
                "protocols",
                &self
                    .protocols
                    .iter()
                    .map(|p| p.capability())
                    .collect::<Vec<_>>(),
            )
            .finish()
    }
}

impl ProtocolRegistry {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
            protocols: Vec::new(),
        }
    }

    pub fn register(&mut self, protocol: Arc<dyn Protocol>) {
        for id in protocol.message_ids() {
            self.handlers.insert(id, protocol.clone());
        }
        self.protocols.push(protocol);
    }

    pub fn capabilities(&self) -> Vec<Capability> {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        for proto in &self.protocols {
            set.insert(proto.capability());
        }
        set.into_iter().collect()
    }

    pub fn get(&self, id: u8) -> Option<Arc<dyn Protocol>> {
        self.handlers.get(&id).cloned()
    }
}
