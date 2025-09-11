use std::time::Duration;

use tokio::time::timeout;

use crate::{
    network::P2PContext,
    rlpx::{
        error::RLPxError,
        extensions::{self, ExtInbound, ExtMessage},
        message::Message,
    },
};
use ethrex_common::H256;

#[derive(Debug)]
pub struct ExtClient {
    ctx: P2PContext,
    capability: &'static str,
    rx: tokio::sync::broadcast::Receiver<ExtInbound>,
}

impl ExtClient {
    pub fn new(ctx: P2PContext, capability: &'static str) -> Result<Self, RLPxError> {
        let rx = extensions::subscribe_extension(capability)?;
        Ok(Self {
            ctx,
            capability,
            rx,
        })
    }

    pub async fn broadcast(&self, code: u8, payload: Vec<u8>) -> Result<(), RLPxError> {
        let msg = Message::Ext(ExtMessage::new(self.capability, code, payload)?);
        self.ctx.broadcast_message(msg).await
    }

    pub async fn send_to_peer(
        &self,
        peer: H256,
        code: u8,
        payload: Vec<u8>,
    ) -> Result<(), RLPxError> {
        let msg = Message::Ext(ExtMessage::new(self.capability, code, payload)?);
        self.ctx.send_to_peer(peer, msg).await
    }

    pub async fn recv_next(&mut self, dur: Option<Duration>) -> Result<ExtInbound, RLPxError> {
        match dur {
            Some(d) => Ok(timeout(d, self.rx.recv())
                .await
                .map_err(|_| RLPxError::NotFound("timeout".into()))??),
            None => Ok(self.rx.recv().await?),
        }
    }
}
