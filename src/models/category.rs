use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use crate::schema::categories;

#[derive(Queryable, Serialize, Deserialize, Clone)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = categories)]
pub struct NewCategory {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Deserialize, AsChangeset)]
#[diesel(table_name = categories)]
pub struct UpdateCategory {
    pub name: Option<String>,
    pub description: Option<String>,
}

impl Category {
    pub fn all(conn: &mut MysqlConnection) -> Result<Vec<Category>, diesel::result::Error> {
        categories::table.load::<Category>(conn)
    }

    pub fn find_by_id(conn: &mut MysqlConnection, category_id: i32) -> Result<Category, diesel::result::Error> {
        categories::table.find(category_id).first::<Category>(conn)
    }

    pub fn create(conn: &mut MysqlConnection, new_category: NewCategory) -> Result<Category, diesel::result::Error> {
        diesel::insert_into(categories::table)
            .values(&new_category)
            .execute(conn)?;

        categories::table
            .order(categories::id.desc())
            .first::<Category>(conn)
    }

    pub fn update(conn: &mut MysqlConnection, category_id: i32, update_data: UpdateCategory) -> Result<Category, diesel::result::Error> {
        diesel::update(categories::table.find(category_id))
            .set(&update_data)
            .execute(conn)?;

        categories::table.find(category_id).first::<Category>(conn)
    }

    pub fn delete(conn: &mut MysqlConnection, category_id: i32) -> Result<(), diesel::result::Error> {
        diesel::delete(categories::table.find(category_id))
            .execute(conn)?;
        Ok(())
    }
}

