// Example to maintain a balance ledger: 
decl_storage! { 
    trait Store for Module<T: Trait> as Balances { 
        Balances: map T::AccountId => Balance; 
    } 
} 

// Example: To define and use a custom event in Substrate: 
decl_event! { 
    pub enum Event<T> where AccountId = <T as system::Trait>::AccountId { 
        Transfer(AccountId, AccountId, Balance), 
    } 
} 