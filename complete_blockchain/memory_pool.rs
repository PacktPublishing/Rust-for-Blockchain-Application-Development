use crate::Transaction;
use data_encoding::HEXLOWER;
use std::collections::HashMap;
use std::sync::RwLock;

/// ( K -> txid_hex, V => Transaction )
pub struct MemoryPool {
    inner: RwLock<HashMap<String, Transaction>>,
}

impl MemoryPool {
    pub fn new() -> MemoryPool {
        MemoryPool {
            inner: RwLock::new(HashMap::new()),
        }
    }

    pub fn contains(&self, txid_hex: &str) -> bool {
        self.inner.read().unwrap().contains_key(txid_hex)
    }

    pub fn add(&self, tx: Transaction) {
        let txid_hex = HEXLOWER.encode(tx.get_id());
        self.inner.write().unwrap().insert(txid_hex, tx);
    }

    pub fn get(&self, txid_hex: &str) -> Option<Transaction> {
        if let Some(tx) = self.inner.read().unwrap().get(txid_hex) {
            return Some(tx.clone());
        }
        None
    }

    pub fn remove(&self, txid_hex: &str) {
        let mut inner = self.inner.write().unwrap();
        inner.remove(txid_hex);
    }

    pub fn get_all(&self) -> Vec<Transaction> {
        let inner = self.inner.read().unwrap();
        let mut txs = vec![];
        for (_, v) in inner.iter() {
            txs.push(v.clone());
        }
        return txs;
    }

    pub fn len(&self) -> usize {
        self.inner.read().unwrap().len()
    }
}


pub struct BlockInTransit {
    inner: RwLock<Vec<Vec<u8>>>,
}

impl BlockInTransit {
    pub fn new() -> BlockInTransit {
        BlockInTransit {
            inner: RwLock::new(vec![]),
        }
    }

    pub fn add_blocks(&self, blocks: &[Vec<u8>]) {
        let mut inner = self.inner.write().unwrap();
        for hash in blocks {
            inner.push(hash.to_vec());
        }
    }

    pub fn first(&self) -> Option<Vec<u8>> {
        let inner = self.inner.read().unwrap();
        if let Some(block_hash) = inner.first() {
            return Some(block_hash.to_vec());
        }
        None
    }

    pub fn remove(&self, block_hash: &[u8]) {
        let mut inner = self.inner.write().unwrap();
        if let Some(idx) = inner.iter().position(|x| x.eq(block_hash)) {
            inner.remove(idx);
        }
    }

    pub fn clear(&self) {
        let mut inner = self.inner.write().unwrap();
        inner.clear();
    }

    pub fn len(&self) -> usize {
        self.inner.read().unwrap().len()
    }
}