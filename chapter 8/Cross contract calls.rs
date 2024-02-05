use near_sdk::{env, near_bindgen, Promise}; 
#[near_bindgen] 
pub struct ContractA {} 

#[near_bindgen] 
impl ContractA { 
    pub fn call_contract_b(&self, account_id: String, amount: u128) { 
        let promise = Promise::new(account_id) 
            .function_call(b"do_something".to_vec(), vec![], amount, env::prepaid_gas() - 10); 
        promise.then(env::promise_result); 
    } 
} 

use near_sdk::{env, near_bindgen}; 
#[near_bindgen] 
pub struct ContractB {} 
#[near_bindgen] 
impl ContractB { 
    pub fn do_something(&self) { 
        // Perform some action 
        env::log(b"Doing something..."); 
    } 
} 