use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use crate::schema::carts;

#[derive(Queryable, Serialize, Deserialize, Clone)]
pub struct Cart {
    pub id: i32,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = carts)]
pub struct NewCart {
    pub user_id: i32,
}

impl Cart {
    pub fn find_by_user_id(conn: &mut MysqlConnection, user_id: i32) -> Result<Option<Cart>, diesel::result::Error> {
        carts::table
            .filter(carts::user_id.eq(user_id))
            .first::<Cart>(conn)
            .optional()
    }

    pub fn find_or_create(conn: &mut MysqlConnection, user_id: i32) -> Result<Cart, diesel::result::Error> {
        match Self::find_by_user_id(conn, user_id)? {
            Some(cart) => Ok(cart),
            None => {
                let new_cart = NewCart { user_id };
                diesel::insert_into(carts::table)
                    .values(&new_cart)
                    .execute(conn)?;

                carts::table
                    .order(carts::id.desc())
                    .first::<Cart>(conn)
            }
        }
    }

    pub fn find_by_id(conn: &mut MysqlConnection, cart_id: i32) -> Result<Cart, diesel::result::Error> {
        carts::table.find(cart_id).first::<Cart>(conn)
    }
}

