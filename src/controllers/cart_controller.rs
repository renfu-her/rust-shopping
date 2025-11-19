use rocket::form::Form;
use rocket::response::Redirect;
use rocket::http::Status;
use rocket_dyn_templates::{Template, context};
use diesel::prelude::*;
use crate::database::Db;
use crate::models::cart::Cart;
use crate::models::cart_item::CartItem;
use crate::middleware::auth::SessionUser;

#[get("/cart")]
pub async fn index(mut conn: DbConn, user: SessionUser) -> Result<Template, Status> {
    let cart = match Cart::find_or_create(&mut *conn, user.id) {
        Ok(cart) => cart,
        Err(_) => return Err(Status::InternalServerError),
    };
    
    let items = match CartItem::with_products(&mut *conn, cart.id) {
        Ok(items) => items,
        Err(_) => return Err(Status::InternalServerError),
    };
    
    let total = match CartItem::calculate_total(&mut *conn, cart.id) {
        Ok(total) => total,
        Err(_) => return Err(Status::InternalServerError),
    };
    
    Ok(Template::render("cart/index", context! {
        items,
        total: total.to_string(),
    }))
}

#[post("/cart/add", data = "<form>")]
pub async fn add(
    form: Form<AddToCartForm>,
    user: SessionUser,
    mut conn: Db,
) -> Result<Redirect, Status> {
    let cart = match Cart::find_or_create(&mut *conn, user.id) {
        Ok(cart) => cart,
        Err(_) => return Err(Status::InternalServerError),
    };
    
    match CartItem::add_or_update(&mut *conn, cart.id, form.product_id, form.quantity) {
        Ok(_) => Ok(Redirect::to("/cart")),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/cart/items/<id>/update", data = "<form>")]
pub async fn update_item(
    id: i32,
    form: Form<UpdateCartItemForm>,
    _user: SessionUser,
    mut conn: Db,
) -> Result<Redirect, Status> {
    match CartItem::update_quantity(&mut *conn, id, form.quantity) {
        Ok(_) => Ok(Redirect::to("/cart")),
        Err(_) => Err(Status::NotFound),
    }
}

#[post("/cart/items/<id>/delete")]
pub async fn delete_item(
    id: i32,
    _user: SessionUser,
    mut conn: Db,
) -> Result<Redirect, Status> {
    match CartItem::delete(&mut *conn, id) {
        Ok(_) => Ok(Redirect::to("/cart")),
        Err(_) => Err(Status::NotFound),
    }
}

#[derive(FromForm)]
pub struct AddToCartForm {
    pub product_id: i32,
    pub quantity: i32,
}

#[derive(FromForm)]
pub struct UpdateCartItemForm {
    pub quantity: i32,
}

