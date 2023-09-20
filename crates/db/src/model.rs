// models.rs

use diesel::prelude::{Insertable, Queryable};

use crate::schema::block;

#[derive(Debug, Queryable)]
pub struct Block {
    pub status: String,
    pub block_hash: String,
    pub block_number: u64,
    pub block_timestamp: u64,
    pub parent_hash: String,
    pub transaction_count: i32,
    pub new_root: String,
    pub sequencer_address: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "block"]
pub struct NewBlock {
    pub status: String,
    pub block_hash: String,
    pub block_number: i64,
    pub block_timestamp: i64,
    pub parent_hash: String,
    pub transaction_count: i32,
    pub new_root: String,
    pub sequencer_address: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

// #[derive(Debug, Queryable)]
// pub struct Transaction {
//     pub transaction_id: i32,
//     pub transaction_hash: String,
//     pub type: String,
//     pub sender_address: String,
//     pub created_at: chrono::NaiveDateTime,
//     pub updated_at: chrono::NaiveDateTime,
// }

// #[derive(Debug, Insertable)]
// #[table_name = "transaction"]
// pub struct NewTransaction<'a> {
//     pub transaction_hash: &'a str,
//     pub type: &'a str,
//     // ... and so on for the fields ...
//     pub sender_address: &'a str,
//     pub created_at: chrono::NaiveDateTime,
//     pub updated_at: chrono::NaiveDateTime,
// }
