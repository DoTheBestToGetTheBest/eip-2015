use std::error::Error;

use crate::spec::EthereumChainSwitchRequest;
use alloy::providers::Provider;
use alloy::providers::ProviderBuilder;

use alloy::transports::Transport;
pub trait WalletUpdateEthereumChain<T, N>: Provider<T, N> + Sized
where
    T: Transport + Clone,
    N: Network,
{
    async fn wallet_update_ethereum_chain(
        &self,
        request: EthereumChainSwitchRequest,
    ) -> Result<bool, Box<dyn Error>> {
        let provider = ProviderBuilder::new().on_http(request.rpc_url.unwrap().parse().unwrap());

        let current_chain = provider.get_chain_id().await.unwrap();
     

        if current_chain == request.chain_name as u64 {
            return Ok(true);
        } else {
        }
        return Err("Chain switch failed".into());
    }
}
impl<P, T, N> WalletUpdateEthereumChain<T, N> for P
where
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
{
}
