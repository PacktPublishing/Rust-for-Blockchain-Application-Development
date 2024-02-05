// Create an implicit account in a transaction 

#[near_bindgen] 
pub fn create_implicit_account(&mut self, account_id: String) { 
    let account_id: ValidAccountId = account_id.try_into().unwrap(); 
    env::log(format!("Creating implicit account: {}", account_id).as_bytes()); 
    // Perform actions with the implicit account 
    // ... 
} 

// Create a full access key 

#[near_bindgen] 
pub fn create_full_access_key(&mut self, public_key: PublicKey) { 
    self.env().key_create( 
        public_key, 
        &access_key::AccessKey { 
            nonce: 0, 
            permission: access_key::Permission::FullAccess, 
        }, 
    ); 
} 

// Create a function call key 
#[near_bindgen] 
pub fn create_function_call_key(&mut self, public_key: PublicKey) { 
    self.env().key_create( 
        public_key, 
        &access_key::AccessKey { 
            nonce: 0, 
            permission: access_key::Permission::FunctionCall { 
                allowance: access_key::FunctionCallPermission { 
                    allowance: 10.into(),  // Maximum number of function call allowances 
                    receiver_id: "receiver_account".to_string(), 
                    method_names: vec!["allowed_method".to_string()], 
                }, 
            }, 
        }, 
    ); 
}

// simple smart contract written in Rust
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize}; 
use near_sdk::collections::Vector; 
use near_sdk::{env, near_bindgen}; 
 
#[near_bindgen] 
#[derive(Default, BorshDeserialize, BorshSerialize)] 
pub struct MyContract { 
    items: Vector<String>, 
} 
#[near_bindgen] 
impl MyContract { 
    pub fn add_item(&mut self, item: String) { 
        self.items.push(&item); 
        env::log(format!("Added item: {}", item).as_bytes()); 
    } 
    pub fn get_items(&self) -> Vec<String> { 
        self.items.to_vec() 
    } 
} 

//storing account metadata
// Import necessary libraries 
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize}; 
use near_sdk::env; 
use near_sdk::near_bindgen; 

// Define the contract structure 
#[near_bindgen] 
#[derive(Default, BorshDeserialize, BorshSerialize)] 
pub struct YourContract { 
    // Declare a field to store account metadata 
    pub account_metadata: Option<String>, 
} 

// Implement methods for storing and retrieving account metadata 
#[near_bindgen] 
impl YourContract { 
    // Method to set or update account metadata 
    pub fn set_account_metadata(&mut self, metadata: String) { 
        self.account_metadata = Some(metadata); 
    } 
    // Method to retrieve account metadata 
    pub fn get_account_metadata(&self) -> Option<String> { 
        self.account_metadata.clone() 
    } 
} 

//smart contract in Rust on the NEAR blockchain, showcasing the management of a contract state. 
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize}; 
use near_sdk::near_bindgen; 
#[near_bindgen] 
#[derive(Default, BorshDeserialize, BorshSerialize)] 
pub struct MyContract { 
    counter: i32, 
} 
#[near_bindgen] 
impl MyContract { 
    pub fn new() -> Self { 
        Self { counter: 0 } 
    } 
    pub fn increment(&mut self) { 
        self.counter += 1; 
    } 
    pub fn get_counter(&self) -> i32 { 
        self.counter 
    } 
} 

// simple transaction using the NEAR SDK

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize}; 
use near_sdk::near_bindgen; 
use near_sdk::env; 
use near_sdk::serde_json::json; 

#[near_bindgen] 
#[derive(Default, BorshDeserialize, BorshSerialize)] 

pub struct MyContract { 
    // Contract state and functionality 
} 

#[near_bindgen] 
impl MyContract { 
    pub fn transfer_tokens(&mut self, receiver_id: String, amount: u64) { 
        let current_account_id = env::current_account_id(); 
        let transfer_action = json!({ 
            "receiver_id": receiver_id, 
            "amount": amount 
        }); 
        let transfer_call = near_sdk::json!({ 
            "contract": current_account_id, 
            "method": "transfer", 
            "args": transfer_action, 
            "gas": env::prepaid_gas() - 100_000_000, // Subtracting 100 TeraGas for additional actions 
            "attached_gas": 5_000_000_000, // 5 GigaGas attached to cover the cost 
            "attached_tokens": amount 
        }); 

        env::promise_create( 
            env::current_account_id(), 
            "do_transfer", 
            transfer_call.to_string().as_bytes(), 
            &receiver_id.into_bytes(), 
            0, 
            0 
        ); 
    } 
} 


// Implementing token transfer safeguards
pub fn transfer_tokens(recipient: AccountId, amount: Balance) { 
    assert!(env::is_valid_account_id(&recipient), "Invalid recipient account"); 
    let sender_balance = self.get_account_balance(env::current_account_id()); 
    assert!(sender_balance >= amount, "Insufficient balance"); 
    // Perform the token transfer 
    token::transfer(&env::current_account_id(), &recipient, amount); 
} 

// Implementing token vesting

pub struct TokenVesting { 
    beneficiary: AccountId, 
    start_timestamp: u64, 
    duration: u64, 
    total_tokens: Balance, 
} 

impl TokenVesting { 
    pub fn release_tokens(&mut self) { 
        let current_timestamp = env::block_timestamp(); 
        let elapsed_time = current_timestamp - self.start_timestamp; 
        if elapsed_time >= self.duration { 
            token::transfer(&env::current_account_id(), &self.beneficiary, self.total_tokens); 
        } else { 
            let tokens_to_release = (self.total_tokens * elapsed_time) / self.duration; 
            token::transfer(&env::current_account_id(), &self.beneficiary, tokens_to_release); 
        } 
    } 
} 

// Storage options 

// Declare a vector of u64 elements 
let mut my_vector: Vec<u64> = Vec::new(); 
// Add elements to the vector 
my_vector.push(10); 
my_vector.push(20); 
my_vector.push(30); 

// Access elements in the vector 
let second_element = my_vector[1]; 

// Declare a LookupSet of string elements 
let mut my_lookupset: LookupSet<String> = LookupSet::new(); 
// Add elements to the LookupSet 
my_lookupset.insert("apple".to_string()); 
my_lookupset.insert("banana".to_string()); 
// Check membership 
let contains_apple = my_lookupset.contains("apple".to_string()); 

// Declare an UnorderedSet of u32 elements 
let mut my_unorderedset: UnorderedSet<u32> = UnorderedSet::new(); 
// Add elements to the UnorderedSet 
my_unorderedset.insert(1); 
my_unorderedset.insert(2); 
// Iterate over the elements 
for element in my_unorderedset.iter() { 
    // Process each element 
} 

// Declare a LookupMap with string keys and u64 values 

let mut my_lookupmap: LookupMap<String, u64> = LookupMap::new(); 
// Add key-value pairs to the LookupMap 
my_lookupmap.insert("key1".to_string(), 10); 
my_lookupmap.insert("key2".to_string(), 20); 

// Access values based on keys 
let value = my_lookupmap.get("key1".to_string()); 

// Declare an UnorderedMap with u32 keys and string values 
let mut my_unorderedmap: UnorderedMap<u32, String> = UnorderedMap::new(); 
// Add key-value pairs to the UnorderedMap 
my_unorderedmap.insert(1, "value1".to_string()); 
my_unorderedmap.insert(2, "value2".to_string()); 
// Iterate over the key-value pairs 
for (key, value) in my_unorderedmap.iter() { 
    // Process each key-value pair 
} 

// Declare a TreeMap with u64 keys and string values 
let mut my_treemap: TreeMap<u64, String> = TreeMap::new(); 
// Add key-value pairs to the TreeMap 
my_treemap.insert(3, "value3".to_string()); 
my_treemap.insert(1, "value1".to_string()); 
my_treemap.insert(2, "value2".to_string()); 
// Iterate over the key-value pairs in sorted order 
for (key, value) in my_treemap.iter() { 
    // Process each key-value pair 
} 

