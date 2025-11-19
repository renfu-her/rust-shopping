use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use bigdecimal::BigDecimal;
use crate::schema::orders;

#[derive(Queryable, Serialize, Deserialize, Clone)]
pub struct Order {
    pub id: i32,
    pub user_id: i32,
    pub total_amount: BigDecimal,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = orders)]
pub struct NewOrder {
    pub user_id: i32,
    pub total_amount: BigDecimal,
    pub status: String,
}

#[derive(Deserialize, AsChangeset)]
#[diesel(table_name = orders)]
pub struct UpdateOrder {
    pub status: Option<String>,
}

impl Order {
    pub fn create(conn: &mut MysqlConnection, new_order: NewOrder) -> Result<Order, diesel::result::Error> {
        diesel::insert_into(orders::table)
            .values(&new_order)
            .execute(conn)?;

        orders::table
            .order(orders::id.desc())
            .first::<Order>(conn)
    }

    pub fn find_by_id(conn: &mut MysqlConnection, order_id: i32) -> Result<Order, diesel::result::Error> {
        orders::table.find(order_id).first::<Order>(conn)
    }

    pub fn by_user_id(conn: &mut MysqlConnection, user_id: i32) -> Result<Vec<Order>, diesel::result::Error> {
        orders::table
            .filter(orders::user_id.eq(user_id))
            .order(orders::created_at.desc())
            .load::<Order>(conn)
    }

    pub fn update_status(conn: &mut MysqlConnection, order_id: i32, status: String) -> Result<Order, diesel::result::Error> {
        diesel::update(orders::table.find(order_id))
            .set(orders::status.eq(status))
            .execute(conn)?;

        orders::table.find(order_id).first::<Order>(conn)
    }
}

