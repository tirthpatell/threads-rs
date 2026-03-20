use serde::{Deserialize, Serialize};

use super::ids::UserId;

/// A Threads user profile with app-scoped data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub username: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Maps to `threads_profile_picture_url` in the API.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        alias = "threads_profile_picture_url"
    )]
    pub profile_pic_url: Option<String>,
    /// Maps to `threads_biography` in the API.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        alias = "threads_biography"
    )]
    pub biography: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(default)]
    pub followers_count: i64,
    #[serde(default)]
    pub media_count: i64,
    #[serde(default)]
    pub is_verified: bool,
}

/// A public Threads user profile (via `threads_profile_discovery` scope).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicUser {
    pub username: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_picture_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub biography: Option<String>,
    #[serde(default)]
    pub is_verified: bool,
    #[serde(default)]
    pub follower_count: i64,
    #[serde(default)]
    pub likes_count: i64,
    #[serde(default)]
    pub quotes_count: i64,
    #[serde(default)]
    pub replies_count: i64,
    #[serde(default)]
    pub reposts_count: i64,
    #[serde(default)]
    pub views_count: i64,
}
