use alloc::string::String;
use casper_types::{ContractPackageHash, U256};

use crate::address::Address;

pub enum BridgePoolEvent {
    // event dispatched in case liquidity was added
    BridgeLiquidityAdded {
        actor: Address,
        token: ContractPackageHash,
        amount: U256,
    },
    BridgeLiquidityRemoved {
        actor: Address,
        token: ContractPackageHash,
        amount: U256,
    },
    // event dispatched in case of swap
    BridgeSwap {
        actor: Address,
        token: ContractPackageHash,
        target_network: U256,
        // client address
        target_address: String,
        amount: U256,
    },
    // event dispatched in case of transfer by signature has happened
    TransferBySignature {
        signer: String,
        receiver: String,
        token: ContractPackageHash,
        amount: U256,
    },
}
