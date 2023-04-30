use crate::models::user::User;
use chrono::prelude::*;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct FilteredUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub photo: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for FilteredUser {
    fn from(u: User) -> FilteredUser {
        FilteredUser {
            id: u.id.to_string(),
            name: u.name,
            email: u.email,
            role: u.role,
            photo: u.photo,
            created_at: u.created_at.unwrap(),
            updated_at: u.updated_at.unwrap(),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct UserData {
    pub user: FilteredUser,
}

#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub status: String,
    pub data: UserData,
}
