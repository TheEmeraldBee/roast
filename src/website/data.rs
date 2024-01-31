use std::{collections::VecDeque, time::Duration};

use rocket::{
    http::CookieJar,
    response::{status::Unauthorized, stream::TextStream},
    tokio::{select, time::interval},
    *,
};

use crate::{
    auth::{logged_in, PermissionType},
    COMMAND,
};

#[get("/log")]
pub async fn data_log(
    cookies: &CookieJar<'_>,
    mut shutdown: Shutdown,
) -> Result<TextStream![String], Unauthorized<String>> {
    if logged_in(cookies) != Some(PermissionType::Admin) {
        return Err(response::status::Unauthorized(
            "User not logged in".to_string(),
        ));
    }

    Ok(TextStream! {
        let mut interval = interval(Duration::from_millis(2000));

        let text = COMMAND.read().expect("COMMAND failed to read").text.clone();
        yield text.join("");

        let mut last_line;

        if text.is_empty() {
            last_line = "".to_string();
        } else {
            last_line = text[text.len() - 1].clone()
        }

        loop {
            select! {
                _ = interval.tick() => {
                    let text = COMMAND.read().expect("COMMAND failed to read").text.clone();
                    let mut result: VecDeque<String> = VecDeque::new();

                    let mut check = text.len();
                    if check == 0 {
                        continue;
                    }

                    check -= 1;
                    loop {
                        if text[check] != last_line {
                            result.push_front(text[check].clone());
                        }

                        if check == 0 || text[check] == last_line {
                            if let Some(s) = result.iter().last() {
                                last_line = s.to_string();
                            }

                            break;
                        }

                        check -= 1;

                    }

                    yield match result.into_iter().reduce(|a, x| a + &x){
                        Some(s) => s,
                        None => "".to_string()
                    }
                },
                _ = &mut shutdown => {
                    yield "Server Shutdown".to_string();
                    break;
                }
            }
        }
    })
}
