// models/user_model.rs
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use super::model::{ConnectionData, DBConnection, CRUD};
use crate::prelude::Error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Thing,
    pub cid: String,
    pub name: String,
    pub avatar: String,
    pub email: String,
    pub password: String,
    pub creation_date: String,
    pub is_visible: bool,
    pub is_inactive: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserCreate {
    pub cid: String,
    pub name: String,
    pub avatar: String,
    pub email: String,
    pub password: String,
    pub creation_date: String,
    pub is_visible: bool,
    pub is_inactive: bool,
}

#[async_trait::async_trait]
impl CRUD<User, UserCreate> for User {
    async fn init_table(db: DBConnection) -> Result<(), Error> {
        let sql = "DEFINE TABLE users SCHEMAFULL;\
                DEFINE FIELD email ON users TYPE string ASSERT is::email($value);\
                DEFINE INDEX email ON TABLE users COLUMNS email UNIQUE;\
                DEFINE FIELD cid ON users TYPE string;\
                DEFINE INDEX cid ON TABLE users COLUMNS cid UNIQUE;\
                DEFINE FIELD name ON users TYPE string;\
                DEFINE FIELD avatar ON users TYPE string;\
                DEFINE FIELD password ON users TYPE string;\
                DEFINE FIELD creation_date ON users TYPE string;\
                DEFINE FIELD is_visible ON users TYPE bool;\
                DEFINE FIELD is_inactive ON users TYPE bool;";
        db.query(sql).await?;

        Ok(())
    }
}
