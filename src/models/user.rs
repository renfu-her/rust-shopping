use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::NaiveDateTime;
use crate::schema::users;

#[derive(Queryable, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Deserialize)]
pub struct RegisterUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn create(conn: &mut MysqlConnection, user_data: RegisterUser) -> Result<User, diesel::result::Error> {
        let password_hash = hash(user_data.password, DEFAULT_COST)
            .map_err(|_| diesel::result::Error::NotFound)?;
        
        let new_user = NewUser {
            username: user_data.username,
            email: user_data.email,
            password_hash,
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(conn)?;

        users::table
            .order(users::id.desc())
            .first::<User>(conn)
    }

    pub fn find_by_username(conn: &mut MysqlConnection, username: &str) -> Result<User, diesel::result::Error> {
        users::table
            .filter(users::username.eq(username))
            .first::<User>(conn)
    }

    pub fn find_by_email(conn: &mut MysqlConnection, email: &str) -> Result<User, diesel::result::Error> {
        users::table
            .filter(users::email.eq(email))
            .first::<User>(conn)
    }

    pub fn find_by_id(conn: &mut MysqlConnection, user_id: i32) -> Result<User, diesel::result::Error> {
        users::table
            .find(user_id)
            .first::<User>(conn)
    }

    pub fn verify_password(&self, password: &str) -> bool {
        verify(password, &self.password_hash).unwrap_or(false)
    }
}

