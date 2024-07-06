use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

table! {
    user_user (user_id) {
        user_id -> Uuid,
        email -> Varchar,
        username -> Varchar,
        password -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        phone_number -> Varchar,
        photo_profile -> Nullable<Varchar>,
        is_client -> Bool,
        is_staff -> Bool,
        is_superuser -> Bool,
        is_active -> Bool,
        last_login -> Nullable<Timestamp>,
        date_updated -> Timestamp,
        date_joined -> Timestamp,
    }
}

table! {
    blocks (id) {
        id -> Uuid,
        previous_hash -> Varchar,
        timestamp -> Timestamp,
        nonce -> BigInt,
        hash -> Varchar,
    }
}

table! {
    transactions (id) {
        id -> Uuid,
        block_id -> Uuid,
        sender -> Varchar,
        receiver -> Varchar,
        amount -> BigInt,
        timestamp -> Timestamp,
    }
}

joinable!(transactions -> blocks (block_id));
allow_tables_to_appear_in_same_query!(user_user, blocks, transactions);
