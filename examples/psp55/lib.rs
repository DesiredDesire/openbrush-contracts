#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod PSP55 {
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::psp55::*,
        traits::{
            Storage,
            String,
        },
    };

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct Contract {
        #[storage_field]
        psp55: psp55::Data,
    }

    impl Transfer for Contract {}

    impl PSP55 for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Contract| {})
        }
    }
}
