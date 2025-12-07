use super::dto::UserDtoResponse;
use crate::{
    shared::{
        config::config::DATABASE_NAME,
        dto::response::{http_bad_request, http_internal_server_error, http_no_content, http_ok},
    },
    users::{
        dto::{CreateUserDtoRequest, UpdateUserDtoRequest, UserIdDtoResponse},
        users_model::{User, USERS_COLLECTION},
        users_service,
    },
};
use actix_web::{delete, get, patch, post, web, HttpResponse};
use log::error;
use mongodb::{
    bson::{doc, oid::ObjectId, to_document, Bson},
    options::{FindOneAndUpdateOptions, FindOptions, ReturnDocument},
    Client, Collection,
};

/// REST API controller for user management.
///
/// All routes are prefixed with `/users` as specified in main.rs via `web::scope("/users")`.
///
/// # Routes
/// - `GET /users` - Get all users
/// - `GET /users/{id}` - Get user by ID
/// - `POST /users` - Create new user
/// - `PATCH /users/{id}` - Update user by ID
/// - `DELETE /users/{id}` - Delete user by ID

#[get("")]
async fn get_all(client: web::Data<Client>) -> HttpResponse {
    let collection: Collection<User> = client.database(DATABASE_NAME).collection(USERS_COLLECTION);

    // Fetch from the network with batch size of 100 elements per network call
    let find_opts: FindOptions = FindOptions::builder().batch_size(100).build();
    let cursor = collection.find(doc! {}).with_options(find_opts).await;

    let mut cursor = match cursor {
        Ok(cursor) => cursor,
        Err(e) => {
            error!("Error running find: {}", e);
            return http_internal_server_error("Database query error".into());
        }
    };

    let mut users: Vec<UserDtoResponse> = Vec::new();
    // Retrieve and deserialize user data from cursor
    while cursor.advance().await.unwrap_or_else(|_| false) {
        let current = cursor.deserialize_current();
        match current {
            Ok(user) => {
                let tmp = UserDtoResponse {
                    id: user._id.to_hex(),
                    first_name: user.first_name,
                    last_name: user.last_name,
                    email: user.email,
                    age: user.age,
                };
                users.push(tmp)
            }
            Err(err) => error!("Not valid user; {}", err),
        }
    }

    // Alternative approach using futures (commented out)
    // while let Some(result) = cursor.try_next().await.unwrap_or_else(|err| {
    //     error!("Not valid user: {}", err);
    //     None
    // }) {
    //     users.push(result);
    // }

    http_ok(users)
}

#[get("{id}")]
async fn get_by_id(client: web::Data<Client>, id: web::Path<String>) -> HttpResponse {
    let id = id.into_inner();
    let object_id = ObjectId::parse_str(&id).unwrap_or_default();
    let collection: Collection<User> = client.database(DATABASE_NAME).collection(USERS_COLLECTION);

    match collection.find_one(doc! { "_id": object_id }).await {
        Ok(Some(user)) => http_ok(UserDtoResponse {
            id: user._id.to_hex(),
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            age: user.age,
        }),
        Ok(None) => http_bad_request(format!("User not found for id {}", id)),
        Err(err) => {
            error!("{}", err);
            http_internal_server_error(format!("Generic error finding id {}", id))
        }
    }
}

#[post("")]
async fn create(client: web::Data<Client>, dto: web::Json<CreateUserDtoRequest>) -> HttpResponse {
    let collection: Collection<User> = client.database(DATABASE_NAME).collection(USERS_COLLECTION);

    let user = User {
        _id: ObjectId::new(),
        first_name: dto.first_name.clone(),
        last_name: dto.last_name.clone(),
        email: dto.email.clone(),
        age: dto.age,
    };

    let can_continue = match collection.find_one(doc! { "email": &dto.email }).await {
        Ok(Some(_)) => false,
        Ok(None) => true,
        Err(err) => {
            error!("{}", err);
            false
        }
    };
    if !can_continue {
        return http_bad_request("Already exists".into());
    }

    let insert_result = match collection.insert_one(user).await {
        Ok(res) => res,
        Err(e) => {
            error!("Error inserting user: {}", e);
            return http_internal_server_error("Failed to insert user".into());
        }
    };
    // Extract and return the inserted ObjectId
    match insert_result.inserted_id {
        Bson::ObjectId(oid) => http_ok(UserIdDtoResponse { id: oid.to_hex() }),
        _ => http_internal_server_error("Failed to insert user".into()),
    }
}

#[patch("{id}")]
async fn update_by_id(
    client: web::Data<Client>,
    id: web::Path<String>,
    dto: web::Json<UpdateUserDtoRequest>,
) -> HttpResponse {
    let id: String = id.into_inner(); // Extract ID from path parameter
    let object_id = ObjectId::parse_str(&id).unwrap_or_default();
    let collection: Collection<User> = client.database(DATABASE_NAME).collection(USERS_COLLECTION);

    let update_doc = to_document(&dto);
    if update_doc.is_err() {
        return http_bad_request("Invalid parameters".into());
    }

    let opts = FindOneAndUpdateOptions::builder()
        .upsert(false)
        .return_document(Some(ReturnDocument::After))
        .build();

    match collection
        .find_one_and_update(
            doc! {
                "_id": object_id
            },
            doc! {
                "$set": update_doc.unwrap()
            },
        )
        .with_options(opts)
        .await
    {
        Ok(Some(user)) => http_ok(UserDtoResponse {
            id: user._id.to_hex(),
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            age: user.age,
        }),
        Ok(None) => http_bad_request(format!("Generic error finding id {}", id)),
        Err(err) => {
            error!("{}", err);
            http_internal_server_error(format!("Generic error finding id {}", id))
        }
    }
}

#[delete("{id}")]
async fn delete_by_id(client: web::Data<Client>, id: web::Path<String>) -> HttpResponse {
    let id = id.into_inner();
    let res = users_service::delete_by_id(client, &id).await;

    match res {
        Ok(_) => http_no_content(),
        Err(err) => {
            error!("{}", err);
            http_internal_server_error(format!("Delete failed for id {}", id))
        }
    }
}

/// Service configuration for user routes.
///
/// Registers all user endpoint handlers with the Actix-web application.
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all);
    cfg.service(get_by_id);
    cfg.service(create);
    cfg.service(update_by_id);
    cfg.service(delete_by_id);
}
