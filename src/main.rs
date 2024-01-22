use std::alloc::System;

pub struct Block {
    timestamp: u128,
    transactions: String,
    previous_block_hash: String,
    hash: String,
    height: usize,
    nonce: String,
}

pub struct blockchain {
    blocks :Vec<Block>
}

impl Block {
    pub fn new_block(prev_block_hash: String, data: String, height: usize) -> Result<Block> {
        let timestamp = System::now()
            .duration_since(System::UNIX_EPOCH)?.as_millis();
        let mut block = Block {
            timestamp: timestamp,
            transactions: Vec::new(),
            previous_block_hash,
            hash: String::new(),
            height,
            nonce: 0,
        };
    }
}

fn main() {
    println!("Hello, world!");
}
