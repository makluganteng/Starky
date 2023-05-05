use starknet::providers::jsonrpc::{HttpTransport, JsonRpcClient};
use url::Url;

// #[tokio::main]
// async fn main() {
//     let rpc_client = JsonRpcClient::new(HttpTransport::new(
//         Url::parse("https://starknet-goerli.cartridge.gg/").unwrap(),
//     ));

//     let block_number = rpc_client.block_number().await.unwrap();
//     let block_hash = rpc_client.block_hash_and_number().await.unwrap();
//     println!("{block_number}, {block_hash.block_hash}");
// }

use starknet::{
    core::types::BlockId,
    providers::{Provider, SequencerGatewayProvider},
};

#[tokio::main]
async fn main() {
    let provider = SequencerGatewayProvider::starknet_alpha_goerli();
    let latest_block = provider.get_block(BlockId::Latest).await;
    // println!("{latest_block:#?}");
    let event  = &latest_block.unwrap();
    let transaction = &event.block_hash;
    println!("{transaction:#?}");
    
}