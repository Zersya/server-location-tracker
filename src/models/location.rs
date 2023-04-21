use serde::{Deserialize, Serialize};
use surrealdb::sql::{Id, Thing};

use crate::{databases::connection::Connection, utils::change_to_lowercase, ModelError};

use super::user::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub id: Thing,
    pub latlng: (f64, f64),
    pub speed: f32,
    pub head: f32,
    pub timestamp: i64,
    pub battery: i32,
    pub user: Thing,
}

#[derive(Debug, Deserialize)]
pub struct FormLocation {
    latitude: f64,
    longitude: f64,
    speed: f32,
    head: f32,
    timestamp: i64,
    battery: i32,
    #[serde(with = "change_to_lowercase")]
    username: String,
}

#[derive(Debug, Deserialize)]
pub struct RequestGetLocation {
    #[serde(with = "change_to_lowercase")]
    pub username: String,
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

    pub async fn get_all(username: &String) -> Result<Vec<Self>, ModelError> {
        let conn = match Connection::connect().await {
            Ok(result) => result,
            Err(err) => {
                return Err(ModelError {
                    status: 1001,
                    message: err.message,
                })
            }
        };

        let mut builder = "SELECT * FROM locations";
        if username.len() > 0 {
            builder = "SELECT * FROM locations WHERE user = $user";
        }

        let result = match conn
            .query(builder)
            .bind((
                "user",
                Thing {
                    tb: "users".to_string(),
                    id: Id::String(username.to_string()),
                },
            ))
            .await
        {
            Ok(mut result) => result.take(0),
            Err(err) => {
                return Err(ModelError {
                    status: 1101,
                    message: err.to_string(),
                })
            }
        };

        let locations = match result {
            Ok(result) => result,
            Err(err) => {
                return Err(ModelError {
                    status: 1101,
                    message: err.to_string(),
                })
            }
        };

        Ok(locations)
    }
}
