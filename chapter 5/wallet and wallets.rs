// wallet 
use ring::signature::{EcdsaKeyPair, KeyPair, ECDSA_P256_SHA256_FIXED_SIGNING};
use serde::{Deserialize, Serialize};

const VERSION: u8 = 0x00;
pub const ADDRESS_CHECK_SUM_LEN: usize = 4;

#[derive(Clone, Serialize, Deserialize)]
pub struct Wallet {
    pkcs8: Vec<u8>,
    public_key: Vec<u8>,
}

impl Wallet {
    
    pub fn new() -> Wallet {
        let pkcs8 = crate::new_key_pair();
        let key_pair =
            EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, pkcs8.as_ref()).unwrap();
        let public_key = key_pair.public_key().as_ref().to_vec();
        Wallet { pkcs8, public_key }
    }

   
    pub fn get_address(&self) -> String {
        let pub_key_hash = hash_pub_key(self.public_key.as_slice());
        let mut payload: Vec<u8> = vec![];
        payload.push(VERSION);
        payload.extend(pub_key_hash.as_slice());
        let checksum = checksum(payload.as_slice());
        payload.extend(checksum.as_slice());
        // version + pub_key_hash + checksum
        crate::base58_encode(payload.as_slice())
    }

    pub fn get_public_key(&self) -> &[u8] {
        self.public_key.as_slice()
    }

    pub fn get_pkcs8(&self) -> &[u8] {
        self.pkcs8.as_slice()
    }
}


pub fn hash_pub_key(pub_key: &[u8]) -> Vec<u8> {
    let pub_key_sha256 = crate::sha256_digest(pub_key);
    crate::ripemd160_digest(pub_key_sha256.as_slice())
}


fn checksum(payload: &[u8]) -> Vec<u8> {
    let first_sha = crate::sha256_digest(payload);
    let second_sha = crate::sha256_digest(first_sha.as_slice());
    second_sha[0..ADDRESS_CHECK_SUM_LEN].to_vec()
}


pub fn validate_address(address: &str) -> bool {
    let payload = crate::base58_decode(address);
    let actual_checksum = payload[payload.len() - ADDRESS_CHECK_SUM_LEN..].to_vec();
    let version = payload[0];
    let pub_key_hash = payload[1..payload.len() - ADDRESS_CHECK_SUM_LEN].to_vec();

    let mut target_vec = vec![];
    target_vec.push(version);
    target_vec.extend(pub_key_hash);
    let target_checksum = checksum(target_vec.as_slice());
    actual_checksum.eq(target_checksum.as_slice())
}


pub fn convert_address(pub_hash_key: &[u8]) -> String {
    let mut payload: Vec<u8> = vec![];
    payload.push(VERSION);
    payload.extend(pub_hash_key);
    let checksum = checksum(payload.as_slice());
    payload.extend(checksum.as_slice());
    crate::base58_encode(payload.as_slice())
}

// wallets

use crate::Wallet;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Read, Write};

pub const WALLET_FILE: &str = "wallet.dat";

pub struct Wallets {
    wallets: HashMap<String, Wallet>,
}

impl Wallets {
    pub fn new() -> Wallets {
        let mut wallets = Wallets {
            wallets: HashMap::new(),
        };
        wallets.load_from_file();
        return wallets;
    }

 
    pub fn create_wallet(&mut self) -> String {
        let wallet = Wallet::new();
        let address = wallet.get_address();
        self.wallets.insert(address.clone(), wallet);
        self.save_to_file();
        return address;
    }

    pub fn get_addresses(&self) -> Vec<String> {
        let mut addresses = vec![];
        for (address, _) in &self.wallets {
            addresses.push(address.clone())
        }
        return addresses;
    }

   
    pub fn get_wallet(&self, address: &str) -> Option<&Wallet> {
        if let Some(wallet) = self.wallets.get(address) {
            return Some(wallet);
        }
        None
    }

 
    pub fn load_from_file(&mut self) {
        let path = current_dir().unwrap().join(WALLET_FILE);
        if !path.exists() {
            return;
        }
        let mut file = File::open(path).unwrap();
        let metadata = file.metadata().expect("unable to read metadata");
        let mut buf = vec![0; metadata.len() as usize];
        let _ = file.read(&mut buf).expect("buffer overflow");
        let wallets = bincode::deserialize(&buf[..]).expect("unable to deserialize file data");
        self.wallets = wallets;
    }

   
    fn save_to_file(&self) {
        let path = current_dir().unwrap().join(WALLET_FILE);
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(&path)
            .expect("unable to open wallet.dat");
        let mut writer = BufWriter::new(file);
        let wallets_bytes = bincode::serialize(&self.wallets).expect("unable to serialize wallets");
        writer.write(wallets_bytes.as_slice()).unwrap();
        let _ = writer.flush();
    }
}

