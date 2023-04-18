use once_cell::sync::Lazy;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

use crate::ConnectionError;

static DB: Surreal<Client> = Surreal::init();

#[derive(Clone)]
pub struct Connection {}

impl Connection {
    // pub async fn new() -> Self {
    //     // Connect to the server
    //     let db = Surreal::new::<Ws>("127.0.0.1:8000")
    //         .await
    //         .expect("Unable to connect SurrealDB");

    //     // Signin as a namespace, database, or root user
    //     db.signin(Root {
    //         username: "root",
    //         password: "root",
    //     })
    //     .await
    //     .expect("Something Error on Authenticated Failed");

    //     // Select a specific namespace / database
    //     db.use_ns("developer")
    //         .use_db("location_trackers")
    //         .await
    //         .expect("Something Error on NameServer or DB");

    //     return Connection { db };
    // }

    pub async fn connect() -> Result<Surreal<Client>, ConnectionError> {
        let db_host = std::env::var("DB_HOST").expect("Unable to load DB Host");
        let db_username = std::env::var("DB_USERNAME").expect("Unable to load DB Username");
        let db_password = std::env::var("DB_PASSWORD").expect("Unable to load DB Password");

        match DB.connect::<Ws>(db_host).await {
            Ok(result) => result,
            Err(err) => {
                return Err(ConnectionError {
                    message: format!("Something error on connect: {}", err.to_string()).to_string(),
                });
            }
        };

        match DB
            .signin(Root {
                username: db_username.as_str(),
                password: db_password.as_str(),
            })
            .await
        {
            Ok(result) => result,
            Err(_) => {
                return Err(ConnectionError {
                    message: "Something Error on Authenticated Failed".to_string(),
                })
            }
        };

        // Select a specific namespace / database
        DB.use_ns("developer")
            .use_db("location_trackers")
            .await
            .expect("Something Error on NameServer or DB");

        Ok(DB.clone())
    }
}
