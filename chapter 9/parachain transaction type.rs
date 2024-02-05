// Import necessary dependencies from Substrate 
use frame_system::Module as System; 
use frame_system::RawOrigin; 
use frame_system::ensure_signed; 
use frame_system::ensure_root; 

// Define your custom transaction module 
pub mod my_custom_module { 
    use super::*; 
    // Define the custom transaction struct 
    #[derive(codec::Encode, codec::Decode, Default, Clone, PartialEq)] 
    pub struct MyCustomTransaction { 
        // Define transaction fields here 
        pub sender: AccountId, 
        pub amount: Balance, 
        // Add any other fields you need 
    } 
    // Implement the dispatchable function for your custom transaction 
    decl_module! { 
        pub struct Module<T: Trait> for enum Call where origin: T::Origin { 
            fn my_custom_transaction(origin, transaction: MyCustomTransaction) { 
                let sender = ensure_signed(origin)?; 
                // Your custom logic for processing the transaction here 
                // You can access the fields of transaction, like transaction.sender and transaction.amount 
                // Emit an event or perform other actions as needed 
                // Self::deposit_event(RawEvent::CustomTransactionProcessed(sender, transaction.amount)); 
            } 
        } 
    } 
} 

// Ensure your custom module is included in the runtime configuration 
impl<T: Trait> frame_system::Module<T> { 
    fn dispatch_bypass_filter( 
        origin: T::Origin, 
        _call: T::Call, 
    ) -> dispatch::DispatchResult { 
        ensure_root(origin)?; 
        Ok(()) 
    } 
}