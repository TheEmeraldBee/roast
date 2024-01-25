#![allow(unused_imports)]

pub mod action;
pub mod auth;
pub mod data;

pub use action::*;
pub use auth::*;
pub use data::*;

use rocket::{
    fs::NamedFile,
    http::CookieJar,
    response::{content::RawHtml, Redirect},
    uri, *,
};

use crate::auth::{logged_in, PermissionType};

#[get("/user")]
pub async fn user(cookies: &CookieJar<'_>) -> Result<RawHtml<String>, Redirect> {
    if logged_in(cookies).is_none() {
        Err(Redirect::to(uri!("/")))
    } else {
        Ok(RawHtml(include_str!("../../dist/user.html").to_string()))
    }
}

#[get("/")]
pub async fn index(cookies: &CookieJar<'_>) -> Result<Redirect, RawHtml<String>> {
    match logged_in(cookies) {
        Some(p) => match p {
            PermissionType::Admin => Ok(Redirect::to(uri!("/log"))),
            PermissionType::User => Ok(Redirect::to(uri!("/user"))),
        },
        None => Err(RawHtml(include_str!("../../dist/index.html").to_string())),
    }
}

#[get("/log")]
pub async fn log(cookies: &CookieJar<'_>) -> Result<RawHtml<String>, Redirect> {
    if logged_in(cookies) != Some(PermissionType::Admin) {
        return Err(Redirect::to(uri!("/")));
    }

    Ok(RawHtml(include_str!("../../dist/log.html").to_string()))
}
