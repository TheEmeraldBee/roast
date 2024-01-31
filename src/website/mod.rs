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

use crate::{
    auth::{logged_in, PermissionType},
    html_template, COMMAND,
};

#[get("/user")]
pub async fn user(cookies: &CookieJar<'_>) -> Result<RawHtml<String>, Redirect> {
    if logged_in(cookies).is_none() {
        Err(Redirect::to(uri!("/")))
    } else {
        let login = logged_in(cookies).expect("User should be Some");
        Ok(
            html_template!("../../dist/user.html" => ["{{ status }}" => match COMMAND.read().unwrap().running {
                true => "Server Status: Running",
                false => "Server Status: Stopped"
            }, "{{ user }}" => match login {
                PermissionType::Admin => "Admin",
                PermissionType::User => "User",
            }, "{{ extra }}" => match login {
                    PermissionType::Admin => "<a class=\"cool-link\" href=\"/log\"><button class=\"cool-button\">Console</button></a>",
                    PermissionType::User => ""
                }
            ]),
        )
    }
}

#[get("/")]
pub async fn index(cookies: &CookieJar<'_>) -> Result<Redirect, RawHtml<String>> {
    match logged_in(cookies) {
        Some(p) => match p {
            PermissionType::Admin => Ok(Redirect::to(uri!("/log"))),
            PermissionType::User => Ok(Redirect::to(uri!("/user"))),
        },
        None => Err(html_template!("../../dist/index.html" => ["{{ login }}" => "Welcome!"])),
    }
}

#[get("/log")]
pub async fn log(cookies: &CookieJar<'_>) -> Result<RawHtml<String>, Redirect> {
    if logged_in(cookies) != Some(PermissionType::Admin) {
        return Err(Redirect::to(uri!("/")));
    }

    // Ok(RawHtml(include_str!("../../dist/log.html").to_string()));
    Ok(html_template!("../../dist/log.html" => []))
}
