use crate::rlpx::{
    Message,
    message::RLPxMessage,
    utils::{snappy_compress, snappy_decompress},
};
use bytes::BufMut;
use ethrex_common::{
    Signature,
    types::{Block, batch::Batch},
};
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
    Batch(MojaveBatch),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MojaveBatch {
    batch: Batch,
}

impl MojaveBatch {
    /// Create a new MojaveBatch
    pub fn new(batch: Batch) -> Self {
        Self { batch }
    }

    /// Get the batch
    pub fn batch(&self) -> &Batch {
        &self.batch
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MojaveBlock {
    block: Block,
    signature: Option<Signature>,
}

impl MojaveBlock {
    /// Create a new MojaveBlock
    pub fn new(block: Block, signature: Signature) -> Self {
        Self { block, signature: Some(signature) }
    }

    /// Get the block
    pub fn block(&self) -> &Block {
        &self.block
    }

    /// Get the signature
    pub fn signature(&self) -> Option<&Signature> {
        self.signature.as_ref()
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
            .encode_optional_field(&self.signature)
            .finish();
        let msg_data = snappy_compress(encoded_data)?;
        buf.put_slice(&msg_data);
        Ok(())
    }

    fn decode(msg_data: &[u8]) -> Result<Self, RLPDecodeError> {
        let decompressed_data = snappy_decompress(msg_data)?;
        let decoder = Decoder::new(&decompressed_data)?;
        let (block, decoder) = decoder.decode_field("block")?;
        let (signature, decoder) = decoder.decode_optional_field();
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

impl RLPxMessage for MojaveBatch {
    const CODE: u8 = 0x3;

    fn encode(&self, buf: &mut dyn BufMut) -> Result<(), RLPEncodeError> {
        let mut encoded_data = vec![];

        Encoder::new(&mut encoded_data)
            .encode_field(&self.batch.number)
            .encode_field(&self.batch.first_block)
            .encode_field(&self.batch.last_block)
            .encode_field(&self.batch.state_root)
            .encode_field(&self.batch.privileged_transactions_hash)
            .encode_field(&self.batch.message_hashes)
            .encode_field(&self.batch.blobs_bundle)
            .encode_optional_field(&self.batch.commit_tx)
            .encode_optional_field(&self.batch.verify_tx)
            .finish();

        let msg_data = snappy_compress(encoded_data)?;
        buf.put_slice(&msg_data);
        Ok(())
    }

    fn decode(msg_data: &[u8]) -> Result<Self, RLPDecodeError> {
        let decompressed_data = snappy_decompress(msg_data)?;
        let decoder = Decoder::new(&decompressed_data)?;
        let (number, decoder) = decoder.decode_field::<u64>("number")?;
        let (first_block, decoder) = decoder.decode_field::<u64>("first_block")?;
        let (last_block, decoder) = decoder.decode_field::<u64>("last_block")?;
        let (state_root, decoder) = decoder.decode_field("state_root")?;
        let (privileged_transactions_hash, decoder) =
            decoder.decode_field("privileged_transactions_hash")?;
        let (message_hashes, decoder) = decoder.decode_field("message_hashes")?;
        let (blobs_bundle, decoder) = decoder.decode_field("blobs_bundle")?;
        let (commit_tx, decoder) = decoder.decode_optional_field();
        let (verify_tx, decoder) = decoder.decode_optional_field();

        decoder.finish()?;
        Ok(Self {
            batch: Batch {
                number,
                first_block,
                last_block,
                state_root,
                privileged_transactions_hash,
                message_hashes,
                blobs_bundle,
                commit_tx,
                verify_tx,
            },
        })
    }
}

impl From<MojaveBatch> for crate::rlpx::message::Message {
    fn from(value: MojaveBatch) -> Self {
        MojaveMessage::Batch(value).into()
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
    fn mojave_batch_encode_decode() {
        let batch = Batch {
            number: 1,
            first_block: 1,
            last_block: 10,
            state_root: Default::default(),
            privileged_transactions_hash: Default::default(),
            message_hashes: vec![Default::default(); 3],
            blobs_bundle: Default::default(),
            commit_tx: Some(Default::default()),
            verify_tx: Some(Default::default()),
        };
        let mojave_batch = MojaveBatch::new(batch.clone());

        let mut buf = Vec::new();
        mojave_batch.encode(&mut buf).unwrap();

        let decoded = MojaveBatch::decode(&buf).unwrap();
        assert_eq!(decoded.batch.number, batch.number);
        assert_eq!(decoded.batch.first_block, batch.first_block);
        assert_eq!(decoded.batch.last_block, batch.last_block);
        assert_eq!(decoded.batch.state_root, batch.state_root);
        assert_eq!(
            decoded.batch.privileged_transactions_hash,
            batch.privileged_transactions_hash
        );
        assert_eq!(decoded.batch.message_hashes, batch.message_hashes);
        assert_eq!(decoded.batch.blobs_bundle, batch.blobs_bundle);
        assert_eq!(decoded.batch.commit_tx, batch.commit_tx);
        assert_eq!(decoded.batch.verify_tx, batch.verify_tx);
    }

    #[test]
    fn mojave_message_codes() {
        assert_eq!(MojaveBlock::CODE, 0x1);
        assert_eq!(MojaveProof::CODE, 0x2);
    }
}
