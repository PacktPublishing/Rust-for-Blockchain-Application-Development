// --------------------------------------------------------------------------------------------------
// Getting started with building the Blockchain ( block.rs / proof of work.rs)
// --------------------------------------------------------------------------------------------------

// Block.rs file
use crate::{ProofOfWork, Transaction};
use serde::{Deserialize, Serialize};
use sled::IVec;

#[derive(Clone, Serialize, Deserialize)]
pub struct Block {
    timestamp: i64,                
    pre_block_hash: String,         
    hash: String,                   
    transactions: Vec<Transaction>, 
    nonce: i64,                     
    height: usize,                  
}

impl Block {
   
    pub fn new_block(pre_block_hash: String, transactions: &[Transaction], height: usize) -> Block {
        let mut block = Block {
            timestamp: crate::current_timestamp(),
            pre_block_hash,
            hash: String::new(),
            transactions: transactions.to_vec(),
            nonce: 0,
            height,
        };
       
        let pow = ProofOfWork::new_proof_of_work(block.clone());
        let (nonce, hash) = pow.run();
        block.nonce = nonce;
        block.hash = hash;
        return block;
    }

  
    pub fn deserialize(bytes: &[u8]) -> Block {
        bincode::deserialize(bytes).unwrap()
    }

   
    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap().to_vec()
    }


    pub fn generate_genesis_block(transaction: &Transaction) -> Block {
        let transactions = vec![transaction.clone()];
        return Block::new_block(String::from("None"), &transactions, 0);
    }

    pub fn hash_transactions(&self) -> Vec<u8> {
        let mut txhashs = vec![];
        for transaction in &self.transactions {
            txhashs.extend(transaction.get_id());
        }
        crate::sha256_digest(txhashs.as_slice())
    }

    pub fn get_transactions(&self) -> &[Transaction] {
        self.transactions.as_slice()
    }

    pub fn get_pre_block_hash(&self) -> String {
        self.pre_block_hash.clone()
    }

    pub fn get_hash(&self) -> &str {
        self.hash.as_str()
    }

    pub fn get_hash_bytes(&self) -> Vec<u8> {
        self.hash.as_bytes().to_vec()
    }

    pub fn get_timestamp(&self) -> i64 {
        self.timestamp
    }

    pub fn get_height(&self) -> usize {
        self.height
    }
}

impl From<Block> for IVec {
    fn from(b: Block) -> Self {
        let bytes = bincode::serialize(&b).unwrap();
        Self::from(bytes)
    }
}



// Proof of work.rs

use crate::Block;
use data_encoding::HEXLOWER;
use num_bigint::{BigInt, Sign};
use std::borrow::Borrow;
use std::ops::ShlAssign;

pub struct ProofOfWork {
    block: Block,
    target: BigInt,
}


const TARGET_BITS: i32 = 8;

const MAX_NONCE: i64 = i64::MAX;

impl ProofOfWork {
    pub fn new_proof_of_work(block: Block) -> ProofOfWork {
        let mut target = BigInt::from(1);
       
        target.shl_assign(256 - TARGET_BITS);
        ProofOfWork { block, target }
    }

    fn prepare_data(&self, nonce: i64) -> Vec<u8> {
        let pre_block_hash = self.block.get_pre_block_hash();
        let transactions_hash = self.block.hash_transactions();
        let timestamp = self.block.get_timestamp();
        let mut data_bytes = vec![];
        data_bytes.extend(pre_block_hash.as_bytes());
        data_bytes.extend(transactions_hash);
        data_bytes.extend(timestamp.to_be_bytes());
        data_bytes.extend(TARGET_BITS.to_be_bytes());
        data_bytes.extend(nonce.to_be_bytes());
        return data_bytes;
    }

   
    pub fn run(&self) -> (i64, String) {
        let mut nonce = 0;
        let mut hash = Vec::new();
        println!("Mining the block");
        while nonce < MAX_NONCE {
            let data = self.prepare_data(nonce);
            hash = crate::sha256_digest(data.as_slice());
            let hash_int = BigInt::from_bytes_be(Sign::Plus, hash.as_slice());

          
            if hash_int.lt(self.target.borrow()) {
                println!("{}", HEXLOWER.encode(hash.as_slice()));
                break;
            } else {
                nonce += 1;
            }
        }
        println!();
        return (nonce, HEXLOWER.encode(hash.as_slice()));
    }
}
