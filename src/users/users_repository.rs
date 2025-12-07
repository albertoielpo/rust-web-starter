//! Data access layer for user operations.
//!
//! This repository layer handles all database operations for the User entity.
use actix_web::web;

use mongodb::{
    bson::{doc, oid::ObjectId},
    Client, Collection,
};

use crate::{
    shared::config::config::DATABASE_NAME,
    users::users_model::{User, USERS_COLLECTION},
};
use log::error;

// NOTE: The following repository methods are currently implemented directly in the controller.
// Uncomment and implement these methods to follow the complete repository pattern:
// pub async fn get_all() {}
// pub async fn get_by_id() {}
// pub async fn create() {}
// pub async fn update_by_id() {}

pub async fn delete_by_id(client: web::Data<Client>, id: &str) -> Result<(), String> {
    let object_id = ObjectId::parse_str(&id).unwrap_or_default();
    let collection: Collection<User> = client.database(DATABASE_NAME).collection(USERS_COLLECTION);

    match collection
        .delete_one(doc! {
            "_id": object_id
        })
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => {
            error!("{}", err);
            Err(format!("Delete failed for id {}", id))
        }
    }
}
