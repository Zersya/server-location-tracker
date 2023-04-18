use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::{databases::connection::Connection, ModelError};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Thing,
    pub username: String,
    pub friends: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLocation {
    pub id: Thing,
}

#[derive(Debug, Deserialize)]
pub struct UserAddFriend {
    user_id: String,
    usernames: Vec<String>,
}

impl User {
    pub async fn add_friend(payload: UserAddFriend) -> Result<Self, ModelError> {
        let conn = match Connection::connect().await {
            Ok(result) => result,
            Err(err) => {
                return Err(ModelError {
                    status: 1001,
                    message: err.message,
                })
            }
        };

        let user: User = match conn.select(("users", &payload.user_id)).await {
            Ok(result) => result,
            Err(_) => {
                return Err(ModelError {
                    status: 1404,
                    message: format!("User {:?} tidak ditemukan", &payload.user_id),
                })
            }
        };

        let mut users: Vec<User> = vec![];

        for username in &payload.usernames {
            let is_exist = user.friends.iter().any(|v| v == username);
            if is_exist {
                return Err(ModelError {
                    status: 1404,
                    message: format!("User {:?} sudah berteman", &username),
                });
            }

            let result: User = match conn.select(("users", username)).await {
                Ok(result) => result,
                Err(_) => {
                    return Err(ModelError {
                        status: 1404,
                        message: format!("User {:?} tidak ditemukan", &username),
                    })
                }
            };

            users.push(result)
        }

        let user = User {
            id: user.id,
            username: user.username,
            friends: payload
                .usernames
                .into_iter()
                .map(|v| v)
                .collect::<Vec<String>>(),
        };

        let result = match conn.update(("users", payload.user_id)).content(user).await {
            Ok(result) => result,
            Err(_) => {
                return Err(ModelError {
                    status: 1002,
                    message: format!("Terjadi kesalahan saat menambahkan"),
                })
            }
        };

        Ok(result)
    }
}
