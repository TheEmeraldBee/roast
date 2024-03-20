use std::process::{Command, Stdio};

use rocket::form::Form;
use rocket::http::CookieJar;
use rocket::serde::json::Json;
use rocket::{get, post};
use rocket::{response::Redirect, uri};
use serde::Deserialize;

use crate::auth::logged_in;
use crate::{command::ServerCommand, COMMAND, CONFIG};

#[derive(Deserialize)]
pub struct CommandForm {
    command: String,
}

#[post("/command", data = "<command>", format = "json")]
pub async fn command(command: Json<CommandForm>, cookies: &CookieJar<'_>) -> &'static str {
    if logged_in(cookies) != Some(crate::auth::PermissionType::Admin) {
        return "";
    }

    COMMAND
        .write()
        .expect("COMMAND failed to lock")
        .send_string(command.command.clone())
        .expect("Command failed to send");

    "Complete"
}

#[get("/boot/<com>")]
pub fn boot(cookies: &CookieJar<'_>, com: String) -> Redirect {
    if logged_in(cookies).is_none() {
        return Redirect::to(uri!("/"));
    }

    let mut command = COMMAND.write().expect("COMMAND failed to lock");

    if command.running {
        Redirect::to(uri!("/user"))
    } else {
        let path = match CONFIG.runnables.iter().find(|x| x.name == com) {
            Some(s) => s,
            None => &CONFIG.runnables[0],
        }
        .path
        .clone();

        *command = ServerCommand::new(
            Command::new("sh")
                .arg(path)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed!"),
            200,
        );

        Redirect::to(uri!("/user"))
    }
}

#[get("/kill")]
pub fn kill(cookies: &CookieJar<'_>) -> Redirect {
    if logged_in(cookies).is_none() {
        return Redirect::to(uri!("/"));
    }

    let mut command = COMMAND.write().expect("COMMAND failed to lock");
    command.send_kill();

    Redirect::to(uri!("/log"))
}
