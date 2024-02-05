// Account creation and contract deployment 

const NEAR_RPC_URL: &str = "https://rpc.mainnet.near.org"; 
// Connect to the NEAR network 
let near = near_sdk::connect::connect(near_sdk::Config { 
    network_id: "mainnet".to_string(), 
    node_url: NEAR_RPC_URL.to_string(), 
}); 
// Create a new account 
let new_account = near.create_account("new_account"); 
// Deploy a contract to the new account 
let contract_code = include_bytes!("path/to/contract.wasm"); 
new_account.deploy_contract(contract_code); 

// Interacting with smart contracts: 
// Instantiate a contract object 
let contract = Contract::new(account_id, contract_id, signer); 
// Call a method on the contract 
contract.call_method("method_name", json!({ "param": "value" })); 
// Get contract state 
let state: ContractState = contract.view_method("get_state", json!({})); 

// Handling tokens
// Transfer tokens from one account to another 
let sender = near.get_account("sender_account"); 
let recipient = near.get_account("recipient_account"); 
sender.transfer(&recipient, 100); 
 
// Check token balance 
let balance = recipient.get_balance(); 