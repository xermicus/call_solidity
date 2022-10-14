#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod call_solidity {
    use ink::env::{
        call::{build_call, Call, ExecutionInput, Selector},
        debug_println, DefaultEnvironment,
    };

    #[ink(storage)]
    #[derive(Default)]
    pub struct CallSolidity {}

    impl CallSolidity {
        #[ink(constructor)]
        pub fn new() -> Self {
            Default::default()
        }

        /// Do a proxy call to `callee` and return its result.
        /// The message under `selector` should have exactly one `u32` arguemnt and return a `u32`.
        #[ink(message)]
        pub fn u32_proxy(
            &self,
            callee: AccountId,
            selector: [u8; 4],
            arg: u32,
            max_gas: Option<u64>,
            transfer_value: Option<u128>,
        ) -> u32 {
            let my_return_value = build_call::<DefaultEnvironment>()
                .call_type(
                    Call::new()
                        .callee(callee)
                        .gas_limit(max_gas.unwrap_or(50000000)),
                )
                .transferred_value(transfer_value.unwrap_or_default())
                .exec_input(ExecutionInput::new(Selector::new(selector)).push_arg(arg))
                .returns::<u32>()
                .fire();
            debug_println!("{:?}", my_return_value);
            my_return_value.unwrap()
        }
    }
}
