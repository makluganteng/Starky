CREATE TABLE "block" (
    "block_id" PRIMARY KEY,
    "status" VARCHAR(255) NOT NULL,
    "block_hash" VARCHAR(255) NOT NULL,
    "block_number" INTEGER NOT NULL,
    "block_timestamp" TIMESTAMP NOT NULL,
    "parent_hash" VARCHAR(255) NOT NULL,
    "transaction_count" INTEGER NOT NULL,
    "new_root" VARCHAR(255) NOT NULL,
    "sequencer_address" VARCHAR(255) NOT NULL,
    "created_at" TIMESTAMP NOT NULL,
    "updated_at" TIMESTAMP NOT NULL
)

CREATE TABLE "transaction" (
    "transaction_id" SERIAL PRIMARY KEY,
    "transaction_hash" VARCHAR(255) NOT NULL,
    "type" VARCHAR(100) NOT NULL,
    "sender_address" VARCHAR(255) NOT NULL,
    "created_at" TIMESTAMP NOT NULL,
    "updated_at" TIMESTAMP NOT NULL
)