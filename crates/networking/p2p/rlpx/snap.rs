use super::{
    message::RLPxMessage,
    utils::{snappy_compress, snappy_decompress},
};
use bytes::{BufMut, Bytes};
use ethrex_common::{
    H256, U256,
    constants::{EMPTY_KECCACK_HASH, EMPTY_TRIE_HASH},
    types::AccountState,
};
use ethrex_rlp::{
    decode::RLPDecode,
    encode::RLPEncode,
    error::{RLPDecodeError, RLPEncodeError},
    structs::{Decoder, Encoder},
};

use crate::rlpx::error::RLPxError;
use crate::rlpx::p2p::Capability;
use crate::rlpx::protocol::Protocol;
use crate::rlpx::{SNAP_CAPABILITY_OFFSET, connection::server::Established};

// Snap Capability Messages

#[derive(Debug, Clone)]
pub struct GetAccountRange {
    // id is a u64 chosen by the requesting peer, the responding peer must mirror the value for the response
    pub id: u64,
    pub root_hash: H256,
    pub starting_hash: H256,
    pub limit_hash: H256,
    pub response_bytes: u64,
}

#[derive(Debug, Clone)]
pub struct AccountRange {
    // id is a u64 chosen by the requesting peer, the responding peer must mirror the value for the response
    pub id: u64,
    pub accounts: Vec<AccountRangeUnit>,
    pub proof: Vec<Bytes>,
}

#[derive(Debug, Clone)]
pub struct GetStorageRanges {
    pub id: u64,
    pub root_hash: H256,
    pub account_hashes: Vec<H256>,
    pub starting_hash: H256,
    pub limit_hash: H256,
    pub response_bytes: u64,
}

#[derive(Debug, Clone)]
pub struct StorageRanges {
    pub id: u64,
    pub slots: Vec<Vec<StorageSlot>>,
    pub proof: Vec<Bytes>,
}

#[derive(Debug, Clone)]
pub struct GetByteCodes {
    pub id: u64,
    pub hashes: Vec<H256>,
    pub bytes: u64,
}

#[derive(Debug, Clone)]
pub struct ByteCodes {
    pub id: u64,
    pub codes: Vec<Bytes>,
}

#[derive(Debug, Clone)]
pub struct GetTrieNodes {
    pub id: u64,
    pub root_hash: H256,
    // [[acc_path, slot_path_1, slot_path_2,...]...]
    // The paths can be either full paths (hash) or only the partial path (compact-encoded nibbles)
    pub paths: Vec<Vec<Bytes>>,
    pub bytes: u64,
}

#[derive(Debug, Clone)]
pub struct TrieNodes {
    pub id: u64,
    pub nodes: Vec<Bytes>,
}

impl GetAccountRange {
    pub const CODE: u8 = 0x00;
}

impl RLPxMessage for GetAccountRange {
    fn code(&self) -> u8 {
        Self::CODE
    }
    fn encode(&self, buf: &mut dyn BufMut) -> Result<(), RLPEncodeError> {
        let mut encoded_data = vec![];
        Encoder::new(&mut encoded_data)
            .encode_field(&self.id)
            .encode_field(&self.root_hash)
            .encode_field(&self.starting_hash)
            .encode_field(&self.limit_hash)
            .encode_field(&self.response_bytes)
            .finish();

        let msg_data = snappy_compress(encoded_data)?;
        buf.put_slice(&msg_data);
        Ok(())
    }

    fn decode(msg_data: &[u8]) -> Result<Self, RLPDecodeError> {
        let decompressed_data = snappy_decompress(msg_data)?;
        let decoder = Decoder::new(&decompressed_data)?;
        let (id, decoder) = decoder.decode_field("request-id")?;
        let (root_hash, decoder) = decoder.decode_field("rootHash")?;
        let (starting_hash, decoder) = decoder.decode_field("startingHash")?;
        let (limit_hash, decoder) = decoder.decode_field("limitHash")?;
        let (response_bytes, decoder) = decoder.decode_field("responseBytes")?;
        decoder.finish()?;

        Ok(Self {
            id,
            root_hash,
            starting_hash,
            limit_hash,
            response_bytes,
        })
    }
}

impl AccountRange {
    pub const CODE: u8 = 0x01;
}

impl RLPxMessage for AccountRange {
    fn code(&self) -> u8 {
        Self::CODE
    }
    fn encode(&self, buf: &mut dyn BufMut) -> Result<(), RLPEncodeError> {
        let mut encoded_data = vec![];
        Encoder::new(&mut encoded_data)
            .encode_field(&self.id)
            .encode_field(&self.accounts)
            .encode_field(&self.proof)
            .finish();

        let msg_data = snappy_compress(encoded_data)?;
        buf.put_slice(&msg_data);
        Ok(())
    }

    fn decode(msg_data: &[u8]) -> Result<Self, RLPDecodeError> {
        let decompressed_data = snappy_decompress(msg_data)?;
        let decoder = Decoder::new(&decompressed_data)?;
        let (id, decoder) = decoder.decode_field("request-id")?;
        let (accounts, decoder) = decoder.decode_field("accounts")?;
        let (proof, decoder) = decoder.decode_field("proof")?;
        decoder.finish()?;

        Ok(Self {
            id,
            accounts,
            proof,
        })
    }
}

impl GetStorageRanges {
    pub const CODE: u8 = 0x02;
}

impl RLPxMessage for GetStorageRanges {
    fn code(&self) -> u8 {
        Self::CODE
    }
    fn encode(&self, buf: &mut dyn BufMut) -> Result<(), RLPEncodeError> {
        let mut encoded_data = vec![];
        Encoder::new(&mut encoded_data)
            .encode_field(&self.id)
            .encode_field(&self.root_hash)
            .encode_field(&self.account_hashes)
            .encode_field(&self.starting_hash)
            .encode_field(&self.limit_hash)
            .encode_field(&self.response_bytes)
            .finish();

        let msg_data = snappy_compress(encoded_data)?;
        buf.put_slice(&msg_data);
        Ok(())
    }

    fn decode(msg_data: &[u8]) -> Result<Self, RLPDecodeError> {
        let decompressed_data = snappy_decompress(msg_data)?;
        let decoder = Decoder::new(&decompressed_data)?;
        let (id, decoder) = decoder.decode_field("request-id")?;
        let (root_hash, decoder) = decoder.decode_field("rootHash")?;
        let (account_hashes, decoder) = decoder.decode_field("accountHashes")?;
        let (starting_hash, decoder): (Bytes, _) = decoder.decode_field("startingHash")?;
        let starting_hash = if !starting_hash.is_empty() {
            H256::from_slice(&starting_hash)
        } else {
            Default::default()
        };
        let (limit_hash, decoder): (Bytes, _) = decoder.decode_field("limitHash")?;
        let limit_hash = if !limit_hash.is_empty() {
            H256::from_slice(&limit_hash)
        } else {
            H256([0xFF; 32])
        };
        let (response_bytes, decoder) = decoder.decode_field("responseBytes")?;
        decoder.finish()?;

        Ok(Self {
            id,
            root_hash,
            starting_hash,
            account_hashes,
            limit_hash,
            response_bytes,
        })
    }
}

impl StorageRanges {
    pub const CODE: u8 = 0x03;
}

impl RLPxMessage for StorageRanges {
    fn code(&self) -> u8 {
        Self::CODE
    }
    fn encode(&self, buf: &mut dyn BufMut) -> Result<(), RLPEncodeError> {
        let mut encoded_data = vec![];
        Encoder::new(&mut encoded_data)
            .encode_field(&self.id)
            .encode_field(&self.slots)
            .encode_field(&self.proof)
            .finish();

        let msg_data = snappy_compress(encoded_data)?;
        buf.put_slice(&msg_data);
        Ok(())
    }

    fn decode(msg_data: &[u8]) -> Result<Self, RLPDecodeError> {
        let decompressed_data = snappy_decompress(msg_data)?;
        let decoder = Decoder::new(&decompressed_data)?;
        let (id, decoder) = decoder.decode_field("request-id")?;
        let (slots, decoder) = decoder.decode_field("slots")?;
        let (proof, decoder) = decoder.decode_field("proof")?;
        decoder.finish()?;

        Ok(Self { id, slots, proof })
    }
}

impl GetByteCodes {
    pub const CODE: u8 = 0x04;
}

impl RLPxMessage for GetByteCodes {
    fn code(&self) -> u8 {
        Self::CODE
    }
    fn encode(&self, buf: &mut dyn BufMut) -> Result<(), RLPEncodeError> {
        let mut encoded_data = vec![];
        Encoder::new(&mut encoded_data)
            .encode_field(&self.id)
            .encode_field(&self.hashes)
            .encode_field(&self.bytes)
            .finish();

        let msg_data = snappy_compress(encoded_data)?;
        buf.put_slice(&msg_data);
        Ok(())
    }

    fn decode(msg_data: &[u8]) -> Result<Self, RLPDecodeError> {
        let decompressed_data = snappy_decompress(msg_data)?;
        let decoder = Decoder::new(&decompressed_data)?;
        let (id, decoder) = decoder.decode_field("request-id")?;
        let (hashes, decoder) = decoder.decode_field("hashes")?;
        let (bytes, decoder) = decoder.decode_field("bytes")?;
        decoder.finish()?;

        Ok(Self { id, hashes, bytes })
    }
}

impl ByteCodes {
    pub const CODE: u8 = 0x05;
}

impl RLPxMessage for ByteCodes {
    fn code(&self) -> u8 {
        Self::CODE
    }
    fn encode(&self, buf: &mut dyn BufMut) -> Result<(), RLPEncodeError> {
        let mut encoded_data = vec![];
        Encoder::new(&mut encoded_data)
            .encode_field(&self.id)
            .encode_field(&self.codes)
            .finish();

        let msg_data = snappy_compress(encoded_data)?;
        buf.put_slice(&msg_data);
        Ok(())
    }

    fn decode(msg_data: &[u8]) -> Result<Self, RLPDecodeError> {
        let decompressed_data = snappy_decompress(msg_data)?;
        let decoder = Decoder::new(&decompressed_data)?;
        let (id, decoder) = decoder.decode_field("request-id")?;
        let (codes, decoder) = decoder.decode_field("codes")?;
        decoder.finish()?;

        Ok(Self { id, codes })
    }
}

impl GetTrieNodes {
    pub const CODE: u8 = 0x06;
}

impl RLPxMessage for GetTrieNodes {
    fn code(&self) -> u8 {
        Self::CODE
    }
    fn encode(&self, buf: &mut dyn BufMut) -> Result<(), RLPEncodeError> {
        let mut encoded_data = vec![];
        Encoder::new(&mut encoded_data)
            .encode_field(&self.id)
            .encode_field(&self.root_hash)
            .encode_field(&self.paths)
            .encode_field(&self.bytes)
            .finish();

        let msg_data = snappy_compress(encoded_data)?;
        buf.put_slice(&msg_data);
        Ok(())
    }

    fn decode(msg_data: &[u8]) -> Result<Self, RLPDecodeError> {
        let decompressed_data = snappy_decompress(msg_data)?;
        let decoder = Decoder::new(&decompressed_data)?;
        let (id, decoder) = decoder.decode_field("request-id")?;
        let (root_hash, decoder) = decoder.decode_field("root_hash")?;
        let (paths, decoder) = decoder.decode_field("paths")?;
        let (bytes, decoder) = decoder.decode_field("bytes")?;
        decoder.finish()?;

        Ok(Self {
            id,
            root_hash,
            paths,
            bytes,
        })
    }
}

impl TrieNodes {
    pub const CODE: u8 = 0x07;
}

impl RLPxMessage for TrieNodes {
    fn code(&self) -> u8 {
        Self::CODE
    }
    fn encode(&self, buf: &mut dyn BufMut) -> Result<(), RLPEncodeError> {
        let mut encoded_data = vec![];
        Encoder::new(&mut encoded_data)
            .encode_field(&self.id)
            .encode_field(&self.nodes)
            .finish();

        let msg_data = snappy_compress(encoded_data)?;
        buf.put_slice(&msg_data);
        Ok(())
    }

    fn decode(msg_data: &[u8]) -> Result<Self, RLPDecodeError> {
        let decompressed_data = snappy_decompress(msg_data)?;
        let decoder = Decoder::new(&decompressed_data)?;
        let (id, decoder) = decoder.decode_field("request-id")?;
        let (nodes, decoder) = decoder.decode_field("nodes")?;
        decoder.finish()?;

        Ok(Self { id, nodes })
    }
}

// Intermediate structures

#[derive(Debug, Clone)]
pub struct AccountRangeUnit {
    pub hash: H256,
    pub account: AccountStateSlim,
}

#[derive(Debug, Clone)]
pub struct AccountStateSlim {
    pub nonce: u64,
    pub balance: U256,
    pub storage_root: Bytes,
    pub code_hash: Bytes,
}

#[derive(Debug, Clone)]
pub struct StorageSlot {
    pub hash: H256,
    pub data: U256,
}

impl RLPEncode for AccountRangeUnit {
    fn encode(&self, buf: &mut dyn BufMut) {
        Encoder::new(buf)
            .encode_field(&self.hash)
            .encode_field(&self.account)
            .finish();
    }
}

impl RLPDecode for AccountRangeUnit {
    fn decode_unfinished(rlp: &[u8]) -> Result<(Self, &[u8]), RLPDecodeError> {
        let decoder = Decoder::new(rlp)?;
        let (hash, decoder) = decoder.decode_field("hash")?;
        let (account, decoder) = decoder.decode_field("account")?;
        Ok((Self { hash, account }, decoder.finish()?))
    }
}

impl RLPEncode for AccountStateSlim {
    fn encode(&self, buf: &mut dyn BufMut) {
        Encoder::new(buf)
            .encode_field(&self.nonce)
            .encode_field(&self.balance)
            .encode_field(&self.storage_root)
            .encode_field(&self.code_hash)
            .finish();
    }
}

impl RLPDecode for AccountStateSlim {
    fn decode_unfinished(rlp: &[u8]) -> Result<(Self, &[u8]), RLPDecodeError> {
        let decoder = Decoder::new(rlp)?;
        let (nonce, decoder) = decoder.decode_field("nonce")?;
        let (balance, decoder) = decoder.decode_field("balance")?;
        let (storage_root, decoder) = decoder.decode_field("storage_root")?;
        let (code_hash, decoder) = decoder.decode_field("code_hash")?;
        Ok((
            Self {
                nonce,
                balance,
                storage_root,
                code_hash,
            },
            decoder.finish()?,
        ))
    }
}

impl From<AccountState> for AccountStateSlim {
    fn from(value: AccountState) -> Self {
        let storage_root = if value.storage_root == *EMPTY_TRIE_HASH {
            Bytes::new()
        } else {
            Bytes::copy_from_slice(value.storage_root.as_bytes())
        };
        let code_hash = if value.code_hash == *EMPTY_KECCACK_HASH {
            Bytes::new()
        } else {
            Bytes::copy_from_slice(value.code_hash.as_bytes())
        };
        Self {
            nonce: value.nonce,
            balance: value.balance,
            storage_root,
            code_hash,
        }
    }
}

impl From<AccountStateSlim> for AccountState {
    fn from(value: AccountStateSlim) -> Self {
        let storage_root = if value.storage_root.is_empty() {
            *EMPTY_TRIE_HASH
        } else {
            H256::from_slice(value.storage_root.as_ref())
        };
        let code_hash = if value.code_hash.is_empty() {
            *EMPTY_KECCACK_HASH
        } else {
            H256::from_slice(value.code_hash.as_ref())
        };
        Self {
            nonce: value.nonce,
            balance: value.balance,
            storage_root,
            code_hash,
        }
    }
}

impl RLPEncode for StorageSlot {
    fn encode(&self, buf: &mut dyn BufMut) {
        Encoder::new(buf)
            .encode_field(&self.hash)
            .encode_bytes(&self.data.encode_to_vec())
            .finish();
    }
}

impl RLPDecode for StorageSlot {
    fn decode_unfinished(rlp: &[u8]) -> Result<(Self, &[u8]), RLPDecodeError> {
        let decoder = Decoder::new(rlp)?;
        let (hash, decoder) = decoder.decode_field("hash")?;
        let (data, decoder) = decoder.get_encoded_item()?;
        let data = U256::decode(ethrex_rlp::decode::decode_bytes(&data)?.0)?;
        Ok((Self { hash, data }, decoder.finish()?))
    }
}

/// Minimal implementation of the [`Protocol`] trait for the `snap` capability.
pub struct SnapProtocol;

impl Protocol for SnapProtocol {
    fn capability(&self) -> Capability {
        Capability::snap(1)
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
            SNAP_CAPABILITY_OFFSET + GetAccountRange::CODE,
            SNAP_CAPABILITY_OFFSET + AccountRange::CODE,
            SNAP_CAPABILITY_OFFSET + GetStorageRanges::CODE,
            SNAP_CAPABILITY_OFFSET + StorageRanges::CODE,
            SNAP_CAPABILITY_OFFSET + GetByteCodes::CODE,
            SNAP_CAPABILITY_OFFSET + ByteCodes::CODE,
            SNAP_CAPABILITY_OFFSET + GetTrieNodes::CODE,
            SNAP_CAPABILITY_OFFSET + TrieNodes::CODE,
        ]
    }
}
