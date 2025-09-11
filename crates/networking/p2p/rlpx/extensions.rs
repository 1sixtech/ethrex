use std::{collections::HashMap, sync::RwLock};

use bytes::BufMut;
use once_cell::sync::Lazy;
use tokio::sync::broadcast;

use crate::rlpx::{error::RLPxError, p2p::Capability};
use ethrex_rlp::error::RLPEncodeError;

const EXT_WINDOW: u8 = 0x10;

const RESERVED_WINDOWS: &[(u8, u8)] = &[
    (0x10, 0x1F), // eth (0x10 + [0..0x0F])
    (0x21, 0x2F), // snap (0x21 + [0..0x0E])
    (0x30, 0x3F), // based
];

#[derive(Clone, Debug)]
pub struct ExtMessage {
    pub capability: &'static str,
    pub base_offset: u8,
    pub code: u8,
    pub data: Vec<u8>,
}

impl ExtMessage {
    pub fn new(capability: &'static str, code: u8, data: Vec<u8>) -> Result<Self, RLPxError> {
        let base_offset = registry().get_base_offset(capability)?;
        Ok(ExtMessage {
            capability,
            base_offset,
            code,
            data,
        })
    }

    pub fn encode(&self, buf: &mut dyn BufMut) -> Result<(), RLPEncodeError> {
        buf.put_slice(&self.data);
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct ExtInbound {
    pub peer_id: ethrex_common::H256,
    pub message: ExtMessage,
}

pub fn ext_msg_id(ext: &ExtMessage) -> u8 {
    ext.base_offset.saturating_add(ext.code)
}

pub fn try_decode_ext(msg_id: u8, data: &[u8]) -> Option<ExtMessage> {
    REGISTRY.read().ok().and_then(|r| {
        r.by_window.iter().find_map(|(cap, base)| {
            let start = *base;
            let end = start.saturating_add(EXT_WINDOW - 1);
            if msg_id >= start && msg_id <= end {
                Some(ExtMessage {
                    capability: cap,
                    base_offset: *base,
                    code: msg_id - *base,
                    data: data.to_vec(),
                })
            } else {
                None
            }
        })
    })
}

pub fn register_extension_capability(
    capability: &'static str,
    base_offset: u8,
) -> Result<(), RLPxError> {
    registry().register(capability, base_offset)
}

pub fn subscribe_extension(capability: &str) -> Result<broadcast::Receiver<ExtInbound>, RLPxError> {
    registry().subscribe(capability)
}

pub fn publish_inbound(msg: ExtInbound) {
    let _ = registry().publish(msg);
}

pub fn registered_capabilities() -> Vec<Capability> {
    REGISTRY
        .read()
        .ok()
        .map(|r| {
            r.by_window
                .keys()
                .filter_map(|name| Capability::custom(name, 1).ok())
                .collect()
        })
        .unwrap_or_default()
}

struct Registry {
    // capability -> base_offset
    by_window: HashMap<&'static str, u8>,
    // capability -> broadcaster
    broadcasters: HashMap<&'static str, broadcast::Sender<ExtInbound>>,
}

static REGISTRY: Lazy<RwLock<Registry>> = Lazy::new(|| RwLock::new(Registry::new()));

fn registry() -> RegistryGuard {
    RegistryGuard {}
}

struct RegistryGuard;

impl Registry {
    fn new() -> Self {
        Self {
            by_window: HashMap::new(),
            broadcasters: HashMap::new(),
        }
    }
}

impl RegistryGuard {
    fn get_base_offset(&self, capability: &str) -> Result<u8, RLPxError> {
        let r = REGISTRY
            .read()
            .map_err(|_| RLPxError::InternalError("registry poisoned".to_string()))?;
        r.by_window
            .get(capability)
            .copied()
            .ok_or_else(|| RLPxError::BadRequest(format!("Unknown capability: {capability}")))
    }

    fn register(&self, capability: &'static str, base_offset: u8) -> Result<(), RLPxError> {
        let start = base_offset;
        let end = start.saturating_add(EXT_WINDOW - 1);
        for (rs, re) in RESERVED_WINDOWS {
            if !(end < *rs || start > *re) {
                return Err(RLPxError::BadRequest(
                    "Capability window overlaps reserved range".to_string(),
                ));
            }
        }

        let mut w = REGISTRY
            .write()
            .map_err(|_| RLPxError::InternalError("registry poisoned".to_string()))?;
        for (&_, &existing) in w.by_window.iter() {
            let es = existing;
            let ee = es.saturating_add(EXT_WINDOW - 1);
            if !(end < es || start > ee) {
                return Err(RLPxError::BadRequest(
                    "Capability window overlaps existing capability".to_string(),
                ));
            }
        }
        if w.by_window.contains_key(capability) {
            return Err(RLPxError::BadRequest(
                "Capability already registered".to_string(),
            ));
        }
        w.by_window.insert(capability, base_offset);
        let (tx, _rx) = broadcast::channel(1024);
        w.broadcasters.insert(capability, tx);
        Ok(())
    }

    fn subscribe(&self, capability: &str) -> Result<broadcast::Receiver<ExtInbound>, RLPxError> {
        let r = REGISTRY
            .read()
            .map_err(|_| RLPxError::InternalError("registry poisoned".to_string()))?;
        r.broadcasters
            .get(capability)
            .map(|tx| tx.subscribe())
            .ok_or_else(|| {
                RLPxError::BadRequest(format!(
                    "Unknown capability or not registered: {capability}"
                ))
            })
    }

    fn publish(&self, msg: ExtInbound) -> Result<(), RLPxError> {
        let r = REGISTRY
            .read()
            .map_err(|_| RLPxError::InternalError("registry poisoned".to_string()))?;
        if let Some(tx) = r.broadcasters.get(msg.message.capability) {
            let _ = tx.send(msg);
            Ok(())
        } else {
            Err(RLPxError::BadRequest(
                "No broadcaster for capability".to_string(),
            ))
        }
    }
}
