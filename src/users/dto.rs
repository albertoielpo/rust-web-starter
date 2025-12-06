use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserDtoResponse {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserIdDtoResponse {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserDtoRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateUserDtoRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<u8>,
}
