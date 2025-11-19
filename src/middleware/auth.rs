use rocket::request::{FromRequest, Outcome, Request};
use rocket::http::Status;
use rocket::serde::{Serialize, Deserialize};
use crate::models::user::User;
use crate::database::Db;
use diesel::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionUser {
    pub id: i32,
    pub username: String,
    pub email: String,
}

impl SessionUser {
    pub fn from_user(user: &User) -> Self {
        SessionUser {
            id: user.id,
            username: user.username.clone(),
            email: user.email.clone(),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for SessionUser {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookies = req.cookies();
        
        if let Some(cookie) = cookies.get_private("user_id") {
            if let Ok(user_id) = cookie.value().parse::<i32>() {
                let db_outcome = req.guard::<Db>().await;
                match db_outcome {
                    Outcome::Success(mut conn) => {
                        match crate::models::user::User::find_by_id(&mut *conn, user_id) {
                            Ok(user) => Outcome::Success(SessionUser::from_user(&user)),
                            Err(_) => Outcome::Error((Status::Unauthorized, ())),
                        }
                    }
                    _ => Outcome::Error((Status::InternalServerError, ())),
                }
            } else {
                Outcome::Error((Status::Unauthorized, ()))
            }
        } else {
            Outcome::Error((Status::Unauthorized, ()))
        }
    }
}

