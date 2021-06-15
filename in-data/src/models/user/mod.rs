
use chrono::{DateTime, Utc};
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct User {
    pub username: String,
    pub data: UserData,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl Default for User {
    fn default() -> Self {
        Self {
            username: String::new(),
            data: UserData::default(),
            created: Utc::now(),
            updated: Utc::now()
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct UserData {
    pub level: usize,
    pub exp: usize,

}

#[derive(Clone, Default)]
pub struct UserNode {
    user: User,
}

impl UserNode {

    pub fn new(user: User) -> Self {
        Self { user }
    }
}

#[derive(Clone, Debug)]
pub struct UserRelation {
    pub weight: i32,
    pub text: Option<String>,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl Default for UserRelation {
    fn default() -> Self {
        Self {
            created: Utc::now(),
            updated: Utc::now(),
            text: None,
            weight: 0,
        }
    }
}

