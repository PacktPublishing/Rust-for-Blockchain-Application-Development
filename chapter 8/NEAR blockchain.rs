// Contract class in Rust. 
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize}; 
use near_sdk::collections::Vector; 
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault, Promise, StorageUsage}; 

// We start by importing necessary dependencies from the NEAR SDK. 
#[near_bindgen] 
#[derive(Default, BorshDeserialize, BorshSerialize, PanicOnDefault)] 
pub struct MyContract { 
    pub items: Vector<String>, 
} 
// We define the main contract struct MyContract with its associated methods. 
impl MyContract { 
    pub fn new() -> Self { 
        Self { 
            items: Vector::new(b"i".to_vec()), 
        } 
    } 

// The constructor function new() initializes the contract, including the vector of items. 
    pub fn add_item(&mut self, item: String) { 
        self.items.push(&item); 
    } 
    pub fn get_items(&self) -> Vec<String> { 
        self.items.to_vec() 
    } 
}  
#[near_bindgen] 
impl MyContract { 
    pub fn contract_metadata(&self) -> ContractMetadata { 
        ContractMetadata { 
            name: "MyContract".to_string(), 
            version: "1.0.0".to_string(), 
            // Additional metadata fields... 
        } 
    } 
} 

// We define additional contract functions, including contract_metadata(), to provide metadata. 

#[derive(Default, BorshDeserialize, BorshSerialize)] 
pub struct ContractMetadata { 
    pub name: String, 
    pub version: String, 
    // Additional metadata fields... 
}


