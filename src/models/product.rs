use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use bigdecimal::BigDecimal;
use crate::schema::{products, categories};

#[derive(Queryable, Serialize, Deserialize, Clone, Associations)]
#[diesel(belongs_to(Category))]
pub struct Product {
    pub id: i32,
    pub category_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: BigDecimal,
    pub stock: i32,
    pub image_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = products)]
pub struct NewProduct {
    pub category_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: BigDecimal,
    pub stock: i32,
    pub image_url: Option<String>,
}

#[derive(Deserialize, AsChangeset)]
#[diesel(table_name = products)]
pub struct UpdateProduct {
    pub category_id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<BigDecimal>,
    pub stock: Option<i32>,
    pub image_url: Option<String>,
}

#[derive(Serialize)]
pub struct ProductWithCategory {
    #[serde(flatten)]
    pub product: Product,
    pub category: crate::models::category::Category,
}

impl Product {
    pub fn all(conn: &mut MysqlConnection) -> Result<Vec<Product>, diesel::result::Error> {
        products::table.load::<Product>(conn)
    }

    pub fn find_by_id(conn: &mut MysqlConnection, product_id: i32) -> Result<Product, diesel::result::Error> {
        products::table.find(product_id).first::<Product>(conn)
    }

    pub fn find_with_category(conn: &mut MysqlConnection, product_id: i32) -> Result<ProductWithCategory, diesel::result::Error> {
        let product = products::table.find(product_id).first::<Product>(conn)?;
        let category = categories::table.find(product.category_id).first::<crate::models::category::Category>(conn)?;
        Ok(ProductWithCategory { product, category })
    }

    pub fn by_category(conn: &mut MysqlConnection, category_id: i32) -> Result<Vec<Product>, diesel::result::Error> {
        products::table
            .filter(products::category_id.eq(category_id))
            .load::<Product>(conn)
    }

    pub fn search(conn: &mut MysqlConnection, query: &str) -> Result<Vec<Product>, diesel::result::Error> {
        let search_pattern = format!("%{}%", query);
        products::table
            .filter(products::name.like(&search_pattern))
            .or_filter(products::description.like(&search_pattern))
            .load::<Product>(conn)
    }

    pub fn create(conn: &mut MysqlConnection, new_product: NewProduct) -> Result<Product, diesel::result::Error> {
        diesel::insert_into(products::table)
            .values(&new_product)
            .execute(conn)?;

        products::table
            .order(products::id.desc())
            .first::<Product>(conn)
    }

    pub fn update(conn: &mut MysqlConnection, product_id: i32, update_data: UpdateProduct) -> Result<Product, diesel::result::Error> {
        diesel::update(products::table.find(product_id))
            .set(&update_data)
            .execute(conn)?;

        products::table.find(product_id).first::<Product>(conn)
    }

    pub fn delete(conn: &mut MysqlConnection, product_id: i32) -> Result<(), diesel::result::Error> {
        diesel::delete(products::table.find(product_id))
            .execute(conn)?;
        Ok(())
    }

    pub fn update_stock(conn: &mut MysqlConnection, product_id: i32, quantity: i32) -> Result<Product, diesel::result::Error> {
        diesel::update(products::table.find(product_id))
            .set(products::stock.eq(products::stock - quantity))
            .execute(conn)?;

        products::table.find(product_id).first::<Product>(conn)
    }
}

