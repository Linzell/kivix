use log::info;
use surrealdb::{
    engine::remote::ws::Client, engine::remote::ws::Ws, opt::auth::Root, sql::Value, Error, Surreal,
};

use crate::utils::env::get_env_or;

pub trait Creatable: Into<Value> {}
pub trait Patchable: Into<Value> {}

/// A repository for the SurrealDB
/// 
/// ## Fields
/// 
/// DB is the main database
/// 
/// SessionDB is the database for sessions
#[derive(Clone)]
pub struct SurrealDBRepo {
    pub db: Surreal<Client>,
    pub session_db: Surreal<Client>,
}

impl SurrealDBRepo {
    /// Initializes the SurrealDB connection
    /// 
    /// ## Returns
    /// Returns a SurrealDBRepo instance
    /// 
    /// ## Errors
    /// Panics if the database connection fails
    pub async fn init() -> Result<Self, Error> {
        info!("🦋 Connecting database...");

        let db_location = get_env_or("DB_LOCATION", "127.0.0.1:8000");

        let db = Surreal::new::<Ws>(db_location.clone())
            .await
            .expect("DB to connect");

        let db_user = get_env_or("DB_USERNAME", "root");
        let db_pass = get_env_or("DB_PASSWORD", "root");

        info!("📖 Signing in...");

        db.signin(Root {
            username: db_user.as_str(),
            password: db_pass.as_str(),
        })
        .await
        .expect("DB Credentials to be correct");

        let db_namespace = get_env_or("DB_NAMESPACE", "test");
        let db_database = get_env_or("DB_DATABASE", "test");

        db.use_ns(db_namespace.clone())
            .use_db(db_database.clone())
            .await
            .expect("using namespace and db to work");

        let session_db = Surreal::new::<Ws>(db_location)
            .await
            .expect("DB to connect");

        session_db
            .signin(Root {
                username: db_user.as_str(),
                password: db_pass.as_str(),
            })
            .await
            .expect("DB Credentials to be correct");

        session_db
            .use_ns(db_namespace)
            .use_db(db_database)
            .await
            .expect("using namespace and db to work");

        Ok(Self { db, session_db })
    }
}
