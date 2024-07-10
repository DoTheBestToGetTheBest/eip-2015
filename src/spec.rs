use alloy::primitives::{ChainId, U256};
use alloy_chains::NamedChain;

pub struct NativeCurrencyData {
    pub name: NamedChain,
    pub symbol: NamedChain,
    pub decimal: U256,
}

pub struct EthereumChainSwitchRequest {
    pub chain_id: ChainId,
    pub chain_name: NamedChain,
    pub rpc_url: Option<String>,
    pub native_currency: Option<NativeCurrencyData>,
    pub block_explorer_url: Option<NamedChain>,
}
