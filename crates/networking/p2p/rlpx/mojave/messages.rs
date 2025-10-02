use crate::rlpx::{
    Message,
    message::RLPxMessage,
    utils::{snappy_compress, snappy_decompress},
};
use bytes::BufMut;
use ethrex_common::{Signature, types::Block};
use ethrex_rlp::{
    error::{RLPDecodeError, RLPEncodeError},
    structs::{Decoder, Encoder},
};
use serde::{Deserialize, Serialize};

/// TODO: add enum variant on demand.
#[derive(Clone, Debug)]
#[allow(clippy::large_enum_variant)]
pub enum MojaveMessage {
    Block(MojaveBlock),
    Proof(MojaveProof),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MojaveBlock {
    block: Block,
    signature: Signature,
}

impl MojaveBlock {
    /// Create a new MojaveBlock
    pub fn new(block: Block, signature: Signature) -> Self {
        Self { block, signature }
    }

    /// Get the block
    pub fn block(&self) -> &Block {
        &self.block
    }

    /// Get the signature
    pub fn signature(&self) -> &Signature {
        &self.signature
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MojaveProof {
    proof: String,
}

impl MojaveProof {
    /// Create a new MojaveProof
    pub fn new(proof: String) -> Self {
        Self { proof }
    }

    /// Get the proof data
    pub fn proof(&self) -> &str {
        &self.proof
    }
}

impl RLPxMessage for MojaveBlock {
    const CODE: u8 = 0x1;

    fn encode(&self, buf: &mut dyn BufMut) -> Result<(), RLPEncodeError> {
        let mut encoded_data = vec![];
        Encoder::new(&mut encoded_data)
            .encode_field(&self.block)
            .encode_field(&self.signature)
            .finish();
        let msg_data = snappy_compress(encoded_data)?;
        buf.put_slice(&msg_data);
        Ok(())
    }

    fn decode(msg_data: &[u8]) -> Result<Self, RLPDecodeError> {
        let decompressed_data = snappy_decompress(msg_data)?;
        let decoder = Decoder::new(&decompressed_data)?;
        let (block, decoder) = decoder.decode_field("block")?;
        let (signature, decoder) = decoder.decode_field("signature")?;
        decoder.finish()?;
        Ok(Self { block, signature })
    }
}

impl RLPxMessage for MojaveProof {
    const CODE: u8 = 0x2;

    fn encode(&self, buf: &mut dyn BufMut) -> Result<(), RLPEncodeError> {
        let mut encoded_data = vec![];
        Encoder::new(&mut encoded_data)
            .encode_field(&self.proof)
            .finish();
        let msg_data = snappy_compress(encoded_data)?;
        buf.put_slice(&msg_data);
        Ok(())
    }

    fn decode(msg_data: &[u8]) -> Result<Self, RLPDecodeError> {
        let decompressed_data = snappy_decompress(msg_data)?;
        let decoder = Decoder::new(&decompressed_data)?;
        let (proof, decoder) = decoder.decode_field("proof")?;
        decoder.finish()?;
        Ok(Self { proof })
    }
}

impl From<MojaveProof> for crate::rlpx::message::Message {
    fn from(value: MojaveProof) -> Self {
        MojaveMessage::Proof(value).into()
    }
}

impl From<MojaveBlock> for crate::rlpx::message::Message {
    fn from(value: MojaveBlock) -> Self {
        MojaveMessage::Block(value).into()
    }
}

impl From<MojaveMessage> for crate::rlpx::message::Message {
    fn from(value: MojaveMessage) -> Self {
        Message::Mojave(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rlpx::message::RLPxMessage;
    use ethrex_common::{Signature, types::Block};

    #[test]
    fn mojave_block_encode_decode() {
        let block = Block {
            header: Default::default(),
            body: Default::default(),
        };
        let signature = Signature::from_slice(&[0u8; 65]);

        let mojave_block = MojaveBlock::new(block, signature);

        let mut buf = Vec::new();
        mojave_block.encode(&mut buf).unwrap();

        let decoded = MojaveBlock::decode(&buf).unwrap();
        assert_eq!(decoded.block, mojave_block.block);
        assert_eq!(decoded.signature, mojave_block.signature);
    }

    #[test]
    fn mojave_proof_encode_decode() {
        let proof_data = "test_proof_data".to_string();
        let mojave_proof = MojaveProof::new(proof_data.clone());

        let mut buf = Vec::new();
        mojave_proof.encode(&mut buf).unwrap();

        let decoded = MojaveProof::decode(&buf).unwrap();
        assert_eq!(decoded.proof(), proof_data);
    }

    #[test]
    fn mojave_message_codes() {
        assert_eq!(MojaveBlock::CODE, 0x1);
        assert_eq!(MojaveProof::CODE, 0x2);
    }
}
