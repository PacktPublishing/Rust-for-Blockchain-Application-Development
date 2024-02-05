pub use pallet_staking; 
use frame_system::Module as System; 
use pallet_staking::Module as Staking; 

decl_module! { 
    pub struct Module<T: Trait> for enum Call where origin: T::Origin { 
        fn deposit_event() = default; 
        // Define custom blockchain logic using FRAME modules. 
    } 
} 