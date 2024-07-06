use uuid::Uuid;
use serde::{Deserialize, Serialize};
use diesel::Queryable;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub user_id: Uuid,
    pub email: String,
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub photo_profile: Option<String>,
    pub is_client: bool,
    pub is_staff: bool,
    pub is_superuser: bool,
    pub is_active: bool,
    pub last_login: Option<chrono::NaiveDateTime>,
    pub date_updated: chrono::NaiveDateTime,
    pub date_joined: chrono::NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Block {
    pub id: Uuid,
    pub previous_hash: String,
    pub timestamp: chrono::NaiveDateTime,
    pub nonce: i64,
    pub hash: String,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub block_id: Uuid,
    pub sender: String,
    pub receiver: String,
    pub amount: i64,
    pub timestamp: chrono::NaiveDateTime,
}

#[derive(Deserialize)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}
