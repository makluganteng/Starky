use crate::schema::block::dsl::*;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::RunQueryDsl;
use diesel_migrations::embed_migrations;
use dotenv::dotenv;
use lazy_static::lazy_static;
use model::NewBlock;
use r2d2;
use std::env;
pub mod model;
pub mod schema;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Debug, thiserror::Error)]
pub enum CustomError {
    #[error("Failed to cnnect to database")]
    ConnectionError,
}
pub struct Database {
    pub pool: Pool,
}

impl Database {
    pub fn init() -> Self {
        dotenv().expect("Failed to load .env file");
        lazy_static! {
            static ref POOL: Pool = {
                let db_url = env::var("DATABASE_URL").expect("Database url not set in .env");
                let manager = ConnectionManager::<PgConnection>::new(db_url);
                Pool::new(manager).expect("Failed to create db pool")
            };
        }
        lazy_static::initialize(&POOL);
        let database = Database { pool: POOL.clone() };
        let _conn = database
            .get_connection()
            .expect("Failed to get db connection during initialization");
        database
    }

    pub fn get_connection(&self) -> Result<DbConnection, CustomError> {
        self.pool.get().map_err(|e| CustomError::ConnectionError)
    }

    pub fn get_pool(&self) -> Pool {
        self.pool.clone()
    }

    pub fn insert_block(&self, new_block: NewBlock) -> Result<usize, diesel::result::Error> {
        use crate::schema::block::dsl::*; // Import table DSL

        let mut conn = self.get_connection().map_err(|_| {
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UnableToSendCommand,
                Box::new("Failed to get db connection".to_string()),
            )
        })?;

        diesel::insert_into(block)
            .values(&new_block)
            .execute(&mut conn)
    }
}

#[cfg(test)]
mod db {

    use super::*;
    #[test]
    fn test_db_connection() {}
}
