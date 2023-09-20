// schema.rs
use diesel::prelude::*;

table! {
    block (block_number) {
        status -> Varchar,
        block_hash -> Varchar,
        block_number -> Int8,
        block_timestamp -> Int8,
        parent_hash -> Varchar,
        transaction_count -> Int4,
        new_root -> Varchar,
        sequencer_address -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

// table! {
//     transaction (transaction_id) {
//         transaction_id -> Int4,
//         transaction_hash -> Varchar,
//         type -> Varchar,
//         sender_address -> Varchar,
//         created_at -> Timestamp,
//         updated_at -> Timestamp,
//     }
// }
