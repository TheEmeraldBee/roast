#![allow(clippy::blocks_in_conditions)]

use std::{collections::HashMap, sync::Mutex};

use lazy_static::lazy_static;
use rand::Rng;
use rocket::{http::CookieJar, FromForm};

lazy_static! {
    pub static ref AUTHENTICATED_USERS: Mutex<HashMap<Auth, PermissionType>> =
        Mutex::new(HashMap::new());
}

#[derive(Clone, Eq, PartialEq)]
pub enum PermissionType {
    Admin,
    User,
}

#[derive(FromForm)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Auth(pub u64);

pub fn logged_in(cookies: &CookieJar<'_>) -> Option<PermissionType> {
    match cookies.get_private("authentication") {
        Some(t) => match t.value_trimmed().parse::<u64>() {
            Ok(v) => AUTHENTICATED_USERS
                .lock()
                .expect("Users failed to lock")
                .get(&Auth(v))
                .cloned(),
            Err(_) => None,
        },
        None => None,
    }
}

pub fn apply_logout(cookies: &CookieJar) {
    if let Some(t) = cookies.get_private("authentication") {
        if let Ok(v) = t.value_trimmed().parse::<u64>() {
            AUTHENTICATED_USERS
                .lock()
                .expect("Users failed to lock")
                .remove(&Auth(v));
        }
    }
}

impl Auth {
    fn new() -> Self {
        Self(rand::thread_rng().gen())
    }

    pub fn login_user() -> Self {
        let user = Self::new();

        AUTHENTICATED_USERS
            .lock()
            .expect("AUTHENTICATED_USERS failed to lock")
            .insert(user, PermissionType::User);

        user
    }

    pub fn login_admin() -> Self {
        let user = Self::new();

        AUTHENTICATED_USERS
            .lock()
            .expect("AUTHENTICATED_USERS failed to lock")
            .insert(user, PermissionType::Admin);

        user
    }
}
