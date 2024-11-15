use crate::types::EFTest;
use ethereum_rust_core::{types::Genesis, H256};
use ethereum_rust_storage::{EngineType, Store};
use ethereum_rust_vm::{evm_state, EvmState};

pub fn load_initial_state(test: &EFTest) -> (EvmState, H256) {
    let genesis = Genesis::from(test);

    let storage = Store::new("./temp", EngineType::InMemory).expect("Failed to create Store");
    storage.add_initial_state(genesis.clone()).unwrap();

    let parent_hash = genesis.get_block().header.parent_hash;

    (
        evm_state(storage.clone(), parent_hash),
        genesis.get_block().header.compute_block_hash(),
    )
}