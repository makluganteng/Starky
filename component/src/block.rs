use ethers::{types::U256, prelude::k256::elliptic_curve::Field};
use starknet::core::types::{FieldElement, FromStrError};
use thiserror::Error;
struct Block {
    block_number: u32,
    block_hash: FieldElement,
    number_txn: u32,
    number_messages: u32,
    number_events: u32 
}


pub type Result<T> = std::result::Result<T, FromStrError>;

impl Block {
    //constructor when a new block is being recieved from the block chain.
    pub fn new(block_number: u32, block_hash: FieldElement, number_txn: u32, number_messages: u32, number_events: u32) -> Self{
        Self {
            block_number,
            block_hash,
            number_txn,
            number_messages,
            number_events
        }
    }

    //get methods
    pub fn get_block_number(&self) -> u32 {
        self.block_number
    }  

    pub fn get_block_hash(&self) -> FieldElement {
        self.block_hash
    }

    pub fn get_number_txn(&self) -> u32 {
        self.number_txn
    }

    pub fn get_number_messages(&self) -> u32 {
        self.number_messages
    }

    pub fn get_number_events(&self) -> u32 {
        self.number_events
    }
}

mod test {
    use super::*;
    #[test]
    fn new_block(){
        let hash = FieldElement::from_hex_be(
            "049ee3eba8c1600700ee1b87eb599f16716b0b1022947733551fde4050ca6805").unwrap();
        let block = Block::new(48872,hash, 222, 5, 1055);
        assert_eq!(block.get_block_number(),48872);
        assert_eq!(block.get_block_hash(),hash);
    }
}