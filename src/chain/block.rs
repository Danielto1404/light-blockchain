use serde::Serialize;
use crate::chain::transaction::Transaction;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BlockHeader {
    pub timestamp: i64,
    pub nonce: u32,
    pub pre_hash: String,
    pub merkle: String,
    pub difficulty: u32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub count: u32,
    pub transactions: Vec<Transaction>,
}