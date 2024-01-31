use rocket::{
    form::Form,
    http::CookieJar,
    response::{content::RawHtml, status::Unauthorized, Redirect},
    uri, *,
};

use crate::{
    auth::{apply_logout, Auth, Login},
    html_template, CONFIG,
};

#[post("/", data = "<form>")]
pub fn login(form: Form<Login>, cookies: &CookieJar<'_>) -> Result<Redirect, RawHtml<String>> {
    if form.username == CONFIG.admin_user && form.password == CONFIG.admin_pass {
        let auth_cookie = Auth::login_admin();
        cookies.add_private(("authentication", auth_cookie.0.to_string()));
        Ok(Redirect::to(uri!("/log")))
    } else if form.username == CONFIG.main_user && form.password == CONFIG.main_pass {
        let auth_cookie = Auth::login_user();
        cookies.add_private(("authentication", auth_cookie.0.to_string()));
        Ok(Redirect::to(uri!("/user")))
    } else {
        Err(
            html_template!("../../dist/index.html" => ["{{ login }}" => "Username or password is incorrect"]),
        )
    }
}

#[get("/logout")]
pub fn logout(cookies: &CookieJar<'_>) -> Redirect {
    apply_logout(cookies);
    cookies.remove_private("authentication");
    Redirect::to(uri!("/"))
}
