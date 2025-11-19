use rocket::form::Form;
use rocket::response::Redirect;
use rocket::http::CookieJar;
use rocket_dyn_templates::{Template, context};
use diesel::prelude::*;
use crate::database::Db;
use crate::models::user::{User, RegisterUser};
use crate::middleware::auth::SessionUser;

#[get("/login")]
pub fn login_page() -> Template {
    Template::render("auth/login", context! {})
}

#[post("/login", data = "<form>")]
pub async fn login(
    form: Form<LoginForm>,
    cookies: &CookieJar<'_>,
    mut conn: Db,
) -> Result<Redirect, Template> {
    let login_data = form.into_inner();
    
    match User::find_by_username(&mut *conn, &login_data.username) {
        Ok(user) => {
            if user.verify_password(&login_data.password) {
                cookies.add_private(("user_id", user.id.to_string()));
                Ok(Redirect::to("/products"))
            } else {
                Err(Template::render("auth/login", context! {
                    error: "Invalid username or password"
                }))
            }
        }
        Err(_) => {
            Err(Template::render("auth/login", context! {
                error: "Invalid username or password"
            }))
        }
    }
}

#[get("/register")]
pub fn register_page() -> Template {
    Template::render("auth/register", context! {})
}

#[post("/register", data = "<form>")]
pub async fn register(
    form: Form<RegisterForm>,
    cookies: &CookieJar<'_>,
    mut conn: Db,
) -> Result<Redirect, Template> {
    let register_data = form.into_inner();
    
    // 檢查使用者是否已存在
    if User::find_by_username(&mut *conn, &register_data.username).is_ok() {
        return Err(Template::render("auth/register", context! {
            error: "Username already exists"
        }));
    }
    
    if User::find_by_email(&mut *conn, &register_data.email).is_ok() {
        return Err(Template::render("auth/register", context! {
            error: "Email already exists"
        }));
    }
    
    let new_user = RegisterUser {
        username: register_data.username,
        email: register_data.email,
        password: register_data.password,
    };
    
    match User::create(&mut *conn, new_user) {
        Ok(user) => {
            cookies.add_private(("user_id", user.id.to_string()));
            Ok(Redirect::to("/products"))
        }
        Err(_) => {
            Err(Template::render("auth/register", context! {
                error: "Failed to create user"
            }))
        }
    }
}

#[get("/logout")]
pub fn logout(cookies: &CookieJar<'_>) -> Redirect {
    cookies.remove_private("user_id");
    Redirect::to("/login")
}

#[derive(FromForm)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[derive(FromForm)]
pub struct RegisterForm {
    pub username: String,
    pub email: String,
    pub password: String,
}

