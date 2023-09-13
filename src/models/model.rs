use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

use crate::prelude::Error;

/// Database Connection
///
/// ## Type Alias
/// 
/// * `Surreal<Client>` - The database connection
pub type DBConnection = Surreal<Client>;
/// Database Connection Data
/// 
/// ## Type Alias
/// 
/// * `actix_web::web::Data<DBConnection>` - The database connection data
pub type ConnectionData = actix_web::web::Data<DBConnection>;

/// CRUD Trait
/// 
/// ## Methods
/// 
/// * `init_table` - Initialize the table
/// * `create` - Create a new entry in the table
/// * `create_id` - Create a new entry in the table
/// * `get_from_id` - Get entry from the table
/// * `update_replace` - Update an entry in the table
/// * `delete` - Delete an entry from the table
/// 
/// ## Generic Types
/// 
/// * `D` - The data type
/// * `C` - The content type
/// 
/// ## Implementations
#[async_trait]
#[allow(clippy::upper_case_acronyms)]
pub trait CRUD<D, C>
where
    D: Serialize + Send + Sync + for<'de> Deserialize<'de> + 'static,
    C: Serialize + Send + Sync + for<'de> Deserialize<'de> + 'static,
{
    /// Initialize the table
    /// 
    /// ## Arguments
    /// 
    /// * `db` - The database connection
    /// 
    /// ## Returns
    /// 
    /// * `Result<(), Error>` - The result of the operation
    /// 
    /// ## Errors
    /// 
    /// * `Error` - The error returned by the database
    async fn init_table(db: DBConnection) -> Result<(), Error>;

    /// Create a new entry in the table
    /// 
    /// ## Arguments
    /// 
    /// * `db` - The database connection
    /// * `tb` - The table name
    /// * `data` - The data to insert
    /// 
    /// ## Returns
    /// 
    /// * `Result<D, Error>` - The result of the operation
    /// 
    /// ## Errors
    /// 
    /// * `Error` - The error returned by the database
    async fn create(db: ConnectionData, tb: String, data: C) -> Result<D, Error> {
        let res: D = db.create(tb).content(data).await?;

        Ok(res)
    }

    /// Create a new entry in the table
    /// 
    /// ## Arguments
    /// 
    /// * `db` - The database connection
    /// * `id` - The id of the entry
    /// * `data` - The data to insert
    /// 
    /// ## Returns
    ///  
    /// * `Result<Option<D>, Error>` - The result of the operation
    /// 
    /// ## Errors
    /// 
    /// * `Error` - The error returned by the database
    async fn create_id(db: ConnectionData, id: Thing, data: D) -> Result<D, Error> {
        let res: D = db.create(id).content(data).await?;

        Ok(res)
    }

    /// Get entry from the table
    /// 
    /// ## Arguments
    /// 
    /// * `db` - The database connection
    /// * `id` - The id of the entry
    /// 
    /// ## Returns
    /// 
    /// * `Result<Option<D>, Error>` - The result of the operation
    /// 
    /// ## Errors
    /// 
    /// * `Error` - The error returned by the database
    async fn get_from_id(db: ConnectionData, id: Thing) -> Result<Option<D>, Error> {
        let res: Option<D> = db.select(id).await?;

        Ok(res)
    }

    /// Update an entry in the table
    /// 
    /// ## Arguments
    /// 
    /// * `db` - The database connection
    /// * `id` - The id of the entry
    /// * `data` - The data to insert
    /// 
    /// ## Returns
    /// 
    /// * `Result<Option<D>, Error>` - The result of the operation
    /// 
    /// ## Errors
    /// 
    /// * `Error` - The error returned by the database
    async fn update_replace(db: ConnectionData, id: Thing, data: D) -> Result<(), Error> {
        let _: D = db.update(id).content(data).await?;

        Ok(())
    }

    /// Delete an entry from the table
    /// 
    /// ## Arguments
    /// 
    /// * `db` - The database connection
    /// * `id` - The id of the entry
    /// 
    /// ## Returns
    /// 
    /// * `Result<(), Error>` - The result of the operation
    /// 
    /// ## Errors
    /// 
    /// * `Error` - The error returned by the database
    async fn delete(db: ConnectionData, id: Thing) -> Result<(), Error> {
        let _: D = db.delete(id).await?;


        Ok(())
    }
}
