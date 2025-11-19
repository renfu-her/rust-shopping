use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use bigdecimal::BigDecimal;
use crate::schema::{order_items, products};

#[derive(Queryable, Serialize, Deserialize, Clone, Associations)]
#[diesel(belongs_to(crate::models::order::Order))]
#[diesel(belongs_to(crate::models::product::Product))]
pub struct OrderItem {
    pub id: i32,
    pub order_id: i32,
    pub product_id: i32,
    pub quantity: i32,
    pub price: BigDecimal,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = order_items)]
pub struct NewOrderItem {
    pub order_id: i32,
    pub product_id: i32,
    pub quantity: i32,
    pub price: BigDecimal,
}

#[derive(Serialize)]
pub struct OrderItemWithProduct {
    #[serde(flatten)]
    pub order_item: OrderItem,
    pub product: crate::models::product::Product,
}

impl OrderItem {
    pub fn create(conn: &mut MysqlConnection, new_item: NewOrderItem) -> Result<OrderItem, diesel::result::Error> {
        diesel::insert_into(order_items::table)
            .values(&new_item)
            .execute(conn)?;

        order_items::table
            .order(order_items::id.desc())
            .first::<OrderItem>(conn)
    }

    pub fn by_order_id(conn: &mut MysqlConnection, order_id: i32) -> Result<Vec<OrderItem>, diesel::result::Error> {
        order_items::table
            .filter(order_items::order_id.eq(order_id))
            .load::<OrderItem>(conn)
    }

    pub fn with_products(conn: &mut MysqlConnection, order_id: i32) -> Result<Vec<OrderItemWithProduct>, diesel::result::Error> {
        let items = order_items::table
            .filter(order_items::order_id.eq(order_id))
            .load::<OrderItem>(conn)?;

        let mut result = Vec::new();
        for item in items {
            let product = products::table.find(item.product_id).first::<crate::models::product::Product>(conn)?;
            result.push(OrderItemWithProduct {
                order_item: item,
                product,
            });
        }
        Ok(result)
    }
}

