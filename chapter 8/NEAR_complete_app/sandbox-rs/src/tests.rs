use near_workspaces::{types::NearToken, Account, Contract};
use serde_json::json;
use std::{env, fs};
 
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wasm_arg: &str = &(env::args().nth(1).unwrap());
    let wasm_filepath = fs::canonicalize(env::current_dir()?.join(wasm_arg))?;
 
    let worker = near_workspaces::sandbox().await?;
    let wasm = std::fs::read(wasm_filepath)?;
    let contract = worker.dev_deploy(&wasm).await?;
 
    // create accounts
    let account = worker.dev_create_account().await?;
    let alice = account
        .create_subaccount("alice")
        .initial_balance(NearToken::from_near(30))
        .transact()
        .await?
        .into_result()?;
 
    // begin tests
    test_default_message(&alice, &contract).await?;
    test_changes_message(&alice, &contract).await?;
    Ok(())
}
 
async fn test_default_message(
    user: &Account,
    contract: &Contract,
) -> Result<(), Box<dyn std::error::Error>> {
    let greeting: String = user
        .call(contract.id(), "get_greeting")
        .args_json(json!({}))
        .transact()
        .await?
        .json()?;
 
    assert_eq!(greeting, "Hello".to_string());
    println!("      Passed ✅ gets default greeting");
    Ok(())
}
 
async fn test_changes_message(
    user: &Account,
    contract: &Contract,
) -> Result<(), Box<dyn std::error::Error>> {
    user.call(contract.id(), "set_greeting")
        .args_json(json!({"greeting": "Howdy"}))
        .transact()
        .await?
        .into_result()?;
 
    let greeting: String = user
        .call(contract.id(), "get_greeting")
        .args_json(json!({}))
        .transact()
        .await?
        .json()?;
 
    assert_eq!(greeting, "Howdy".to_string());
    println!("      Passed ✅ changes greeting");
    Ok(())
}