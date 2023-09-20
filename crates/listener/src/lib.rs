//things needed to be here are function to listen to new incoming blocks from an RPC.
//step 1 create the struct to initialize a new listener and listen to that RPC

use std::{fs::File, io::Write};

use db::{model::NewBlock, schema::block, Database};
use starknet::{
    core::types::{BlockId, BlockTag, MaybePendingBlockWithTxs},
    providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider},
};
use tokio::time::{sleep, Duration};
use url::Url;

pub struct Listener {
    rpc_client: JsonRpcClient<HttpTransport>,
    db_client: Database,
}

impl Listener {
    pub fn new(url: &str, db: Database) -> Result<Self, url::ParseError> {
        let rpc_client = JsonRpcClient::new(HttpTransport::new(Url::parse(url)?));
        Ok(Listener {
            rpc_client,
            db_client: db,
        })
    }

    pub async fn run(&self) {
        let rpc_listener = &self.rpc_client;
        let db_connect = &self.db_client.get_connection();

        let mut last_block_number: Option<u64> = None;

        loop {
            match rpc_listener.block_number().await {
                Ok(current_block_number) => {
                    if Some(current_block_number) != last_block_number {
                        println!("New block number: {}", current_block_number);
                        match rpc_listener
                            .get_block_with_txs(BlockId::Number(current_block_number))
                            .await
                        {
                            Ok(current_block_data) => {
                                // Process the current block data here
                                // For now, just a placeholder print statement
                                println!("Block data: {:?}", current_block_data);
                                write_block_to_json_file(
                                    &current_block_data,
                                    "./listener/data.json",
                                )
                                .expect("Failed to write");
                                match &current_block_data {
                                    MaybePendingBlockWithTxs::Block(block) => {
                                        let new_block = NewBlock {
                                            status: "pending".to_string(),
                                            block_hash: block.block_hash.to_string(),
                                            block_number: block.block_number.try_into().unwrap(),
                                            block_timestamp: block.timestamp.try_into().unwrap(),
                                            parent_hash: block.parent_hash.to_string(),
                                            transaction_count: block.transactions.len() as i32,
                                            new_root: block.new_root.to_string(),
                                            sequencer_address: block.sequencer_address.to_string(),
                                            created_at: chrono::Local::now().naive_local(),
                                            updated_at: chrono::Local::now().naive_local(),
                                        };
                                        match db_connect {
                                            Ok(conn) => {
                                                match self.db_client.insert_block(new_block) {
                                                    Ok(_) => {
                                                        println!("Block inserted successfully");
                                                    }
                                                    Err(e) => {
                                                        println!("Error inserting block: {}", e);
                                                    }
                                                }
                                            }
                                            Err(e) => {
                                                println!("Error connecting to db: {}", e);
                                            }
                                        }
                                    }

                                    MaybePendingBlockWithTxs::PendingBlock(block) => {}
                                }
                            }
                            Err(e) => {
                                println!(
                                    "Error fetching block data for block number {}: {}",
                                    current_block_number, e
                                );
                            }
                        }
                        last_block_number = Some(current_block_number);
                    }
                }
                Err(e) => {
                    println!("Error fetching the block number: {}", e);
                }
            }
            sleep(Duration::from_secs(5)).await;
        }
    }
}

fn write_block_to_json_file(
    block_data: &MaybePendingBlockWithTxs,
    file_path: &str,
) -> Result<(), serde_json::Error> {
    // Convert block data to JSON string
    let json_data = serde_json::to_string(&block_data)?;

    // Write the JSON string to a file
    let mut file = File::create(file_path).expect("Failed to create file");
    file.write_all(json_data.as_bytes())
        .expect("Failed to write to file");

    Ok(())
}
