use derive_new::new;
use rand::prelude::*;
use std::fmt::Debug;

#[derive(Debug)]
pub enum Role {
    Guest,
    Viewer,
    Creator,
    Admin,
}

#[derive(Debug)]
pub struct User {
    id: u32,
    pub username: String,
    pub role: Role,
}

impl User {
    pub fn new(username: String) -> Result<Self, String> {
        if username == "cdd" {
            return Err("username already exists!".to_owned());
        }
        Ok(Self {
            id: thread_rng().gen_range(0..999999999),
            username,
            role: Role::Creator,
        })
    }
}

impl Default for User {
    fn default() -> Self {
        let id = thread_rng().gen_range(0..99999999);
        Self {
            id,
            username: format!("guest{id}"),
            role: Role::Guest,
        }
    }
}

#[derive(Debug, Default, new)]
pub struct Post {
    content: String,
    #[new(value = "vec![\"hello\".to_owned(),\"world\".to_owned()]")]
    tags: Vec<String>,
    #[new(value = "6173")]
    likes: u32,
    #[new(default)]
    extra: bool,
}
