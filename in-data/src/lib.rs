pub mod graph;
pub mod models;

use std::collections::BTreeMap;
use chrono::{DateTime, Utc};
use actix::prelude::*;

pub fn test() {

}

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

#[derive(Default)]
pub struct UserData {
    pub level: usize,
    pub exp: usize,

}

#[derive(Default)]
pub struct UserNode {
    user: User,
}

impl UserNode {

    pub fn new(user: User) -> Self {
        Self { user }
    }
}

pub struct UserRelation {
    idx: i64,
    user_by: UserNode,
    user_to: UserNode,
    weight: i64,
    metadata: BTreeMap<String, String>,
    created: DateTime<Utc>,
    updated: DateTime<Utc>,
}

impl Default for UserRelation {
    fn default() -> Self {
        Self {
            created: Utc::now(),
            updated: Utc::now(),
            ..Default::default()
        }
    }
}

pub struct UserGraph {
    graph: BTreeMap<u32, UserRelation>,
}

impl UserGraph {

    pub fn add_user(&mut self, user: User) {
        self.graph.insert(0, UserRelation::default());
    }
}

#[cfg(test)]
mod tests {

}
