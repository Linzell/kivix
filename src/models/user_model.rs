// models/user_model.rs
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use super::model::{ConnectionData, DBConnection, CRUD};
use crate::prelude::Error;

/// User Struct
///
/// ## Fields
///
/// * `ID` is the user's unique identifier
/// * `PeerID` is the user's unique identifier
/// * `Name` is the user's name
/// * `Avatar` is the user's avatar
/// * `Email` is the user's email
/// * `PasswordHash` is the user's password hash
/// * `CreationDate` is the user's creation date
/// * `IsVisible` is the user's visibility
/// * `IsInactive` is the user's inactivitys
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Thing,
    pub peer_id: String,
    pub name: String,
    pub avatar: String,
    pub email: String,
    pub password_hash: String,
    pub creation_date: String,
    pub is_visible: bool,
    pub is_inactive: bool,
}

/// User Create Struct
///
/// ## Fields
///
/// * `PeerID` is the user's unique identifier
/// * `Name` is the user's name
/// * `Avatar` is the user's avatar
/// * `Email` is the user's email
/// * `PasswordHash` is the user's password hash
/// * `CreationDate` is the user's creation date
/// * `IsVisible` is the user's visibility
/// * `IsInactive` is the user's inactivitys
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserCreate {
    pub peer_id: String,
    pub name: String,
    pub avatar: String,
    pub email: String,
    pub password_hash: String,
    pub creation_date: String,
    pub is_visible: bool,
    pub is_inactive: bool,
}

#[async_trait::async_trait]
impl CRUD<User, UserCreate> for User {
    /// Initialize the user table
    ///
    /// ## Arguments
    /// ```
    /// db is the database connection
    /// ```
    ///
    /// ## Filters
    /// ```
    /// email is the user's email
    /// peer_id is the user's unique identifier
    /// ```
    ///
    /// ## Returns
    /// Initializes the user table with the given databases
    async fn init_table(db: DBConnection) -> Result<(), Error> {
        let sql = "DEFINE TABLE users SCHEMAFULL;\
                DEFINE FIELD email ON users TYPE string ASSERT is::email($value);\
                DEFINE INDEX email ON TABLE users COLUMNS email UNIQUE;\
                DEFINE FIELD peer_id ON users TYPE string;\
                DEFINE INDEX peer_id ON TABLE users COLUMNS peer_id UNIQUE;\
                DEFINE FIELD name ON users TYPE string;\
                DEFINE FIELD avatar ON users TYPE string;\
                DEFINE FIELD password_hash ON users TYPE string;\
                DEFINE FIELD creation_date ON users TYPE string;\
                DEFINE FIELD is_visible ON users TYPE bool;\
                DEFINE FIELD is_inactive ON users TYPE bool;";
        db.query(sql).await?;

        Ok(())
    }
}

#[allow(dead_code)]
impl User {
    /// Get a user from their Peer ID
    ///
    /// ## Arguments
    ///
    /// * `db` - The database connection
    /// * `peer_id` - The user's peer id
    ///
    /// ## Returns
    ///
    /// * `Result<Option<User>, Error>` - The result of the operation
    ///
    /// ## Errors
    ///
    /// * `Error` - The error returned by the database
    pub async fn get_from_peer_id(
        db: ConnectionData,
        peer_id: &str,
    ) -> Result<Option<User>, Error> {
        let mut res = db
            .query("SELECT * FROM users WHERE peer_id=$id")
            .bind(("id", peer_id))
            .await?;
        let user = res.take(0)?;

        Ok(user)
    }

    /// Get a user from their email
    ///
    /// ## Arguments
    ///
    /// * `db` - The database connection
    /// * `email` - The user's email
    ///
    /// ## Returns
    ///
    /// * `Result<Option<User>, Error>` - The result of the operation
    ///
    /// ## Errors
    ///
    /// * `Error` - The error returned by the database
    pub async fn get_from_email(db: ConnectionData, email: &str) -> Result<Option<User>, Error> {
        let mut res = db
            .query("SELECT * FROM users WHERE email=$email")
            .bind(("email", email))
            .await?;
        let user = res.take(0)?;

        Ok(user)
    }

    /// Verify that the user's password is correct
    ///
    /// ## Arguments
    ///
    /// * `password` - The user's password
    ///
    /// ## Returns
    ///
    /// * `Result<(), argon2::password_hash::Error>` - The result of the operation
    /// 
    /// ## Errors
    /// 
    /// * `argon2::password_hash::Error` - The error returned by the password hashing library
    pub fn verify_password(&self, password: String) -> Result<(), argon2::password_hash::Error> {
        let argon2 = Argon2::default();

        let hash = PasswordHash::new(&self.password_hash)?;

        argon2.verify_password(password.as_bytes(), &hash)
    }
}
