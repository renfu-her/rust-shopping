use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use bigdecimal::BigDecimal;
use crate::schema::{cart_items, products};

#[derive(Queryable, Serialize, Deserialize, Clone, Associations)]
#[diesel(belongs_to(crate::models::cart::Cart))]
#[diesel(belongs_to(crate::models::product::Product))]
pub struct CartItem {
    pub id: i32,
    pub cart_id: i32,
    pub product_id: i32,
    pub quantity: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = cart_items)]
pub struct NewCartItem {
    pub cart_id: i32,
    pub product_id: i32,
    pub quantity: i32,
}

#[derive(Deserialize, AsChangeset)]
#[diesel(table_name = cart_items)]
pub struct UpdateCartItem {
    pub quantity: Option<i32>,
}

#[derive(Serialize)]
pub struct CartItemWithProduct {
    #[serde(flatten)]
    pub cart_item: CartItem,
    pub product: crate::models::product::Product,
}

impl CartItem {
    pub fn by_cart_id(conn: &mut MysqlConnection, cart_id: i32) -> Result<Vec<CartItem>, diesel::result::Error> {
        cart_items::table
            .filter(cart_items::cart_id.eq(cart_id))
            .load::<CartItem>(conn)
    }

    pub fn with_products(conn: &mut MysqlConnection, cart_id: i32) -> Result<Vec<CartItemWithProduct>, diesel::result::Error> {
        let items = cart_items::table
            .filter(cart_items::cart_id.eq(cart_id))
            .load::<CartItem>(conn)?;

        let mut result = Vec::new();
        for item in items {
            let product = products::table.find(item.product_id).first::<crate::models::product::Product>(conn)?;
            result.push(CartItemWithProduct {
                cart_item: item,
                product,
            });
        }
        Ok(result)
    }

    pub fn find_by_cart_and_product(conn: &mut MysqlConnection, cart_id: i32, product_id: i32) -> Result<Option<CartItem>, diesel::result::Error> {
        cart_items::table
            .filter(cart_items::cart_id.eq(cart_id))
            .filter(cart_items::product_id.eq(product_id))
            .first::<CartItem>(conn)
            .optional()
    }

    pub fn add_or_update(conn: &mut MysqlConnection, cart_id: i32, product_id: i32, quantity: i32) -> Result<CartItem, diesel::result::Error> {
        match Self::find_by_cart_and_product(conn, cart_id, product_id)? {
            Some(existing) => {
                let new_quantity = existing.quantity + quantity;
                diesel::update(cart_items::table.find(existing.id))
                    .set(cart_items::quantity.eq(new_quantity))
                    .execute(conn)?;
                cart_items::table.find(existing.id).first::<CartItem>(conn)
            }
            None => {
                let new_item = NewCartItem {
                    cart_id,
                    product_id,
                    quantity,
                };
                diesel::insert_into(cart_items::table)
                    .values(&new_item)
                    .execute(conn)?;

                cart_items::table
                    .filter(cart_items::cart_id.eq(cart_id))
                    .filter(cart_items::product_id.eq(product_id))
                    .first::<CartItem>(conn)
            }
        }
    }

    pub fn update_quantity(conn: &mut MysqlConnection, item_id: i32, quantity: i32) -> Result<CartItem, diesel::result::Error> {
        diesel::update(cart_items::table.find(item_id))
            .set(cart_items::quantity.eq(quantity))
            .execute(conn)?;

        cart_items::table.find(item_id).first::<CartItem>(conn)
    }

    pub fn delete(conn: &mut MysqlConnection, item_id: i32) -> Result<(), diesel::result::Error> {
        diesel::delete(cart_items::table.find(item_id))
            .execute(conn)?;
        Ok(())
    }

    pub fn clear_cart(conn: &mut MysqlConnection, cart_id: i32) -> Result<(), diesel::result::Error> {
        diesel::delete(cart_items::table.filter(cart_items::cart_id.eq(cart_id)))
            .execute(conn)?;
        Ok(())
    }

    pub fn calculate_total(conn: &mut MysqlConnection, cart_id: i32) -> Result<BigDecimal, diesel::result::Error> {
        let items = Self::with_products(conn, cart_id)?;
        let total: BigDecimal = items.iter()
            .map(|item| &item.product.price * BigDecimal::from(item.cart_item.quantity))
            .sum();
        Ok(total)
    }
}

