use serde::{Deserialize, Serialize};
use surrealdb::sql::{Id, Thing};

use crate::{databases::connection::Connection, ModelError};

use super::user::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub id: Thing,
    pub latlng: (f64, f64),
    pub speed: i32,
    pub head: i32,
    pub timestamp: i64,
    pub battery: i32,
    pub user: Thing,
}

#[derive(Serialize, Deserialize)]
pub struct FormLocation {
    latitude: f64,
    longitude: f64,
    speed: i32,
    head: i32,
    timestamp: i64,
    battery: i32,
    username: String,
}

impl Location {
    pub async fn create(payload: FormLocation) -> Result<Self, ModelError> {
        let conn = match Connection::connect().await {
            Ok(result) => result,
            Err(err) => {
                return Err(ModelError {
                    status: 1001,
                    message: err.message,
                })
            }
        };

        let username = &payload.username;

        let user: User = match conn.select(("users", &payload.username)).await {
            Ok(result) => result,
            Err(_) => {
                let user = User {
                    id: Thing {
                        tb: "users".to_string(),
                        id: Id::String(username.into()),
                    },
                    username: payload.username,
                    friends: vec![],
                };

                match conn.create("users").content(user).await {
                    Ok(result) => result,
                    Err(err) => {
                        return Err(ModelError {
                            status: 1101,
                            message: err.to_string(),
                        })
                    }
                }
            }
        };

        let location_created: Location = match conn
            .create("locations")
            .content(Location {
                id: Thing {
                    tb: "locations".to_string(),
                    id: Id::rand(),
                },
                latlng: (payload.latitude, payload.longitude),
                speed: payload.speed,
                head: payload.head,
                timestamp: payload.timestamp,
                battery: payload.battery,
                user: user.id,
            })
            .await
        {
            Ok(result) => result,
            Err(err) => {
                return Err(ModelError {
                    status: 1101,
                    message: err.to_string(),
                })
            }
        };

        Ok(location_created)
    }

    pub async fn update(payload: FormLocation) -> Result<Self, ModelError> {
        todo!()
    }

    pub async fn delete(id: u64) -> Result<Self, ModelError> {
        todo!()
    }

    pub async fn get_all() -> Result<Vec<Self>, ModelError> {
        let conn = match Connection::connect().await {
            Ok(result) => result,
            Err(err) => {
                return Err(ModelError {
                    status: 1001,
                    message: err.message,
                })
            }
        };

        let created: Vec<Location> = match conn.select("locations").await {
            Ok(result) => result,
            Err(err) => {
                return Err(ModelError {
                    status: 1101,
                    message: err.to_string(),
                })
            }
        };

        Ok(created)
    }
}
