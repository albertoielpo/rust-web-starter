//! Business logic layer for user operations.
//!
//! This service layer orchestrates business logic and delegates
//! data access operations to the repository layer.
use actix_web::web;
use mongodb::Client;

use crate::users::users_repository;

pub async fn delete_by_id(client: web::Data<Client>, id: &str) -> Result<(), String> {
    users_repository::delete_by_id(client, id).await
}
