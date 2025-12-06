use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

pub const USERS_COLLECTION: &str = "users";

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub _id: ObjectId, // Option<ObjectId>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub age: Option<u8>,
}
