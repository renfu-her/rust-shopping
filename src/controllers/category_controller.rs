use rocket::form::Form;
use rocket::response::Redirect;
use rocket::http::Status;
use rocket_dyn_templates::{Template, context};
use diesel::prelude::*;
use crate::database::Db;
use crate::models::category::{Category, NewCategory, UpdateCategory};
use crate::middleware::auth::SessionUser;

#[get("/categories")]
pub async fn index(mut conn: Db, user: Option<SessionUser>) -> Template {
    let categories = Category::all(&mut *conn).unwrap_or_default();
    Template::render("categories/index", context! {
        categories,
        user,
    })
}

#[get("/categories/create")]
pub fn create_page(_user: SessionUser) -> Template {
    Template::render("categories/create", context! {})
}

#[post("/categories", data = "<form>")]
pub async fn create(
    form: Form<CategoryForm>,
    _user: SessionUser,
    mut conn: Db,
) -> Result<Redirect, Template> {
    let category_data = form.into_inner();
    
    let new_category = NewCategory {
        name: category_data.name,
        description: Some(category_data.description),
    };
    
    match Category::create(&mut *conn, new_category) {
        Ok(_) => Ok(Redirect::to("/categories")),
        Err(_) => {
            Err(Template::render("categories/create", context! {
                error: "Failed to create category",
            }))
        }
    }
}

#[get("/categories/<id>/edit")]
pub async fn edit_page(id: i32, mut conn: Db, _user: SessionUser) -> Result<Template, Status> {
    match Category::find_by_id(&mut *conn, id) {
        Ok(category) => {
            Ok(Template::render("categories/edit", context! {
                category,
            }))
        }
        Err(_) => Err(Status::NotFound),
    }
}

#[post("/categories/<id>", data = "<form>")]
pub async fn update(
    id: i32,
    form: Form<CategoryForm>,
    _user: SessionUser,
    mut conn: Db,
) -> Result<Redirect, Status> {
    let category_data = form.into_inner();
    
    let update_data = UpdateCategory {
        name: Some(category_data.name),
        description: Some(Some(category_data.description)),
    };
    
    match Category::update(&mut *conn, id, update_data) {
        Ok(_) => Ok(Redirect::to("/categories")),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/categories/<id>/delete")]
pub async fn delete(id: i32, _user: SessionUser, mut conn: Db) -> Result<Redirect, Status> {
    match Category::delete(&mut *conn, id) {
        Ok(_) => Ok(Redirect::to("/categories")),
        Err(_) => Err(Status::NotFound),
    }
}

#[derive(FromForm)]
pub struct CategoryForm {
    pub name: String,
    pub description: String,
}

