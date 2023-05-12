use block::BlockDisplay;
use starknet::{
    core::types::FieldElement,
    providers::jsonrpc::{HttpTransport, JsonRpcClient},
};
use std::{
    fs::File,
    io::{Error, Write},
};
mod block;
use starknet::{
    core::types::BlockId,
    providers::{Provider, SequencerGatewayProvider},
};

#[tokio::main]
async fn main() {
    let provider = SequencerGatewayProvider::starknet_alpha_goerli();
    let latest_block_result = provider
        .get_block(BlockId::Hash(
            FieldElement::from_hex_be(
                "0x720cec5cd0df87c60e79133dbe2869958d5fffd5b703f4c3d22c4648c750cfc",
            )
            .unwrap(),
        ))
        .await;
    // println!("{latest_block:#?}");
    // let event = &latest_block.unwrap();
    match latest_block_result {
        Ok(latest_block) => {
            let block = BlockDisplay::new(
                latest_block.block_number,
                latest_block.block_hash,
                latest_block.transactions.len() as u32,
                0, // Assuming `number_messages` and `number_events` are not available in `latest_block`
                0,
            );

            println!("Block Number: {:?}", block.get_block_number());
            println!("Block Hash: {:?}", block.get_block_hash());
            println!("Number of Transactions: {:?}", block.get_number_txn());
        }
        Err(err) => {
            eprintln!("Error retrieving the latest block: {:?}", err);
        }
    }
}
