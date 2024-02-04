use std::time::SystemTime;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use log::info;
use serde::{Deserialize, Serialize};
use crate::errors::Result;
use crate::blockchain::Blockchain;
use crate::transaction::Transaction;





const TARGET_HEX: usize =4;
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Block {
    timestamp: u128,
    transactions: Vec<Transaction>,
    prev_block_hash: String,
    hash: String,
    height: usize,
    nonce: i32,
}


impl Block {
    pub fn get_hash(&self) ->String {
        self.hash.clone()
    }
    pub fn get_transactions(&self) -> &  Vec<Transaction> {
        &self.transactions
    }
    pub fn get_prev_hsh (&self) ->String {
        self.prev_block_hash.clone()
    }

    pub fn new_genesis_block(coinbase: Transaction) -> Block {
        Block::new_block(String::new(),vec![coinbase], 0).unwrap()
    }

    pub fn new_block(prev_block_hash: String, data:Vec<Transaction> , height: usize) -> Result<Block> {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?.as_millis();
        let mut block = Block {
            timestamp: timestamp,
            transactions: data,
            prev_block_hash,
            hash: String::new(),
            height,
            nonce: 0,
        };
        block.run_proof_of_work()?;
        Ok(block)
    }

    fn run_proof_of_work(&mut self) -> Result<()> {
        info!("mining");
        while !self.validate()? {
            self.nonce += 1;
        }
        let data = self.prepare_hash_data()?;
        let mut hasher = Sha256::new();
        hasher.input(&data[..]);
        self.hash = hasher.result_str();
        Ok(())
    }    
    fn validate(&self) -> Result<bool> {
        let data:Vec<u8> = self.prepare_hash_data()?;
        let mut hasher = Sha256::new();
        hasher.input(&data[..]);
        let mut vec1 = vec![];
        vec1.resize(TARGET_HEX, '0' as u8);
        Ok(&hasher.result_str()[0..TARGET_HEX] == String::from_utf8(vec1)?)


    }
    fn prepare_hash_data(&self) -> Result<Vec<u8>> {
        let content = (
            self.prev_block_hash.clone(),
            self.transactions.clone(),
            self.timestamp,
            self.nonce,
            TARGET_HEX,
        );
        let byte: Vec<u8>= bincode::serialize(&content)?;
        Ok(byte)
    }
}



