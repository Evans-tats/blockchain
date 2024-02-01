use std::collections::HashMap;

use crate::errors::Result;
use crate::blocks::Block;
use crate::transaction::{TXOutput, Transaction};
use log::info;
use rand::seq::index;
use sled::Db;
//const GENSIS_COINBASE: [&str; 1] = ["GenesisBlock"];
const GENSIS_COINBASE: &str = "genesisblock"; 
const TARGET_HEX: usize = 4;
#[derive(Debug,Clone)]
pub struct Blockchain {
    current_hash: String,
    db: sled::Db,
}
pub struct BlockchainIter<'a> {
    current_hash: String,
    bc: &'a Blockchain,
}

impl Blockchain {
    pub fn load_blocks() -> Result<Blockchain> {
        info!("open blockchain");

        let db = sled::open("data/blocks")?;
        let hash = db.get("LAST")?.expect("must create a new blockchain database first");
        info!("found database");
        let lasthash = String::from_utf8(hash.to_vec())?;
        Ok(Blockchain {
            current_hash: lasthash,
            db,
        })
    }
    pub fn create_blockchain(address: String) -> Result<Blockchain> {
        info!("creating blockchain from address");
        let db = sled::open("data/blocks")?;
        let cbtx = Transaction::new_coinbase(address,String::from(
             GENSIS_COINBASE))?;
        let genesis = Block::new_genesis_block(cbtx);
        db.insert(genesis.get_hash(), bincode::serialize(&genesis)?)?;
        db.insert("LAST", genesis.get_hash().as_bytes())?;
        let bc = Blockchain {
            current_hash: genesis.get_hash(),
            db,
        };
        bc.db.flush()?;
        Ok(bc)
        
    }

    fn find_unspent_transactions(&self,address: &str) -> Vec<Transaction> {
        let mut spent_TXOs: HashMap<String, Vec<i32>> = HashMap::new();
        let mut unspent_TXOs: Vec<Transaction>= Vec::new();
        for block in self.iter() {
            for tx in block.get_transactions() {
                for index in 0..tx.vout.len() {
                    if let Some(ids) = spent_TXOs.get(&tx.id) {
                        if ids.contains(&(index as i32)) {
                            continue;
                        }
                    }
                    if tx.vout[index].can_be_unlock_with(address) {
                        unspent_TXOs.push(tx.to_owned())
                    }
                }
                if !tx.is_coinbase() {
                    for i in &tx.vin {
                        if i.can_unlock_output_with(address) {
                            match spent_TXOs.get_mut(&i.txid) {
                                Some(v) => {
                                    v.push(i.vout);
                                }
                                None => {
                                    spent_TXOs.insert(i.txid.clone(), vec![i.vout]);
                            }
                            
                            }
                        }
                    }
                }
            }
        }
        unspent_TXOs
    }
    pub fn find_UTXO(&self, address: &str) -> Vec<TXOutput> {
        let mut utoxs = Vec::<TXOutput>::new();
        let unspend_TXs = self.find_unspent_transactions(address);
        for tx in unspend_TXs {
            for out in &tx.vout {
                if out.can_be_unlock_with(address) {
                    utoxs.push(out.clone());
                }
            }
        }
        utoxs
    }

    pub fn find_spendable_outputs(&self, address: &str,amount: i32,) -> (i32, HashMap<String, Vec<i32>>) {
        let mut unspent_outputs: HashMap<String, Vec<i32>> = HashMap::new();
        let mut accumulated: i32 = 0;
        let unspend_TXs: Vec<Transaction> = self.find_unspent_transactions(address);

        for tx in unspend_TXs {
            for index in 0..tx.vout.len() {
                if tx.vout[index].can_be_unlock_with(address) && accumulated < amount {
                    match unspent_outputs.get_mut(&tx.id) {
                        Some(v) => v.push(index as i32),
                        None => {
                            unspent_outputs.insert(tx.id.clone(), vec![index as i32]);
                        }
                    }
                    accumulated += tx.vout[index].value;
                    if accumulated >= amount {
                        return(accumulated, unspent_outputs);
                    }
                }


            }
        }
        
        (accumulated,unspent_outputs)        
    }
    
    pub fn add_block(&mut self, transaction: Vec<Transaction>) -> Result<()> {
        let lasthash = self.db.get("LAST")?.unwrap();

        let new_block = Block::new_block(String::from_utf8(lasthash.to_vec())?, transaction, TARGET_HEX)?;
        self.db.insert(new_block.get_hash(), bincode::serialize(&new_block)?)?;
        self.db.insert("LAST",new_block.get_hash().as_bytes())?;
        self.current_hash = new_block.get_hash();
        Ok(())
    }
    pub fn iter(&self) ->BlockchainIter {
        BlockchainIter {
            current_hash: self.current_hash.clone(),
            bc: &self,
        }
    }


}
impl<'a> Iterator for BlockchainIter<'a> {
    type Item = Block;
    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(encoded_block) =  self.bc.db.get(&self.current_hash) {
            return match encoded_block {
                Some(b) => {
                    if let Ok(block) = bincode::deserialize::<Block>(&b) {
                        self.current_hash = block.get_prev_hsh();
                        Some(block)
                    }else {
                        None
                    }
                }
                None => None
            };
        }
        None
        
    }
}

mod tests {
    use super::*;

    # [test]
    fn test_blockchain() {
        let mut b=  Blockchain::new().unwrap();
        b.add_block("data".to_string());
        b.add_block("data2".to_string());
        b.add_block("data3".to_string()); 
        for item in b.iter() {
            println!("item: {:?}", item)
        }
        
    }
}