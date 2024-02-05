use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize}; 
use near_sdk::collections::Map; 
use near_sdk::{env, near_bindgen, AccountId}; 

// Import necessary dependencies and modules from the NEAR SDK. 
#[near_bindgen] 
#[derive(Default, BorshDeserialize, BorshSerialize)] 
pub struct UserRegistry { 
    users: Map<AccountId, UserInfo>, 
} 
// Define the UserRegistry smart contract struct, which includes a users field of type Map to store user information. 

#[derive(BorshDeserialize, BorshSerialize)] 
pub struct UserInfo { 
    name: String, 
    age: u32, 
    address: String, 
} 

// Define the UserInfo struct, which represents the data structure for user information, including name, age, and address. 

impl UserRegistry { 
    pub fn new_user(&mut self, name: String, age: u32, address: String) { 
        let caller = env::signer_account_id(); 
        let user_info = UserInfo { 
            name, 
            age, 
            address, 
        }; 
        self.users.insert(&caller, &user_info); 
    } 
    pub fn get_user(&self, user_id: AccountId) -> Option<UserInfo> { 
        self.users.get(&user_id) 
    } 
} 