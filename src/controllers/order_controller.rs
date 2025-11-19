use rocket::response::Redirect;
use rocket::http::Status;
use rocket_dyn_templates::{Template, context};
use diesel::prelude::*;
use bigdecimal::BigDecimal;
use crate::database::Db;
use crate::models::cart::Cart;
use crate::models::cart_item::CartItem;
use crate::models::order::{Order, NewOrder};
use crate::models::order_item::{OrderItem, NewOrderItem};
use crate::models::product::Product;
use crate::middleware::auth::SessionUser;
use crate::utils::pagination::Pagination;

#[get("/orders?<page>")]
pub async fn index(
    page: Option<i64>,
    user: SessionUser,
    mut conn: Db,
) -> Template {
    let current_page = page.unwrap_or(1);
    let per_page = 10;
    
    let total: i64 = crate::schema::orders::table
        .filter(crate::schema::orders::user_id.eq(user.id))
        .count()
        .get_result(&mut *conn)
        .unwrap_or(0);
    
    let pagination = Pagination::new(current_page, per_page, total);
    
    let orders = Order::by_user_id(&mut *conn, user.id).unwrap_or_default();
    
    Template::render("orders/index", context! {
        orders,
        pagination,
    })
}

#[get("/orders/<id>")]
pub async fn show(
    id: i32,
    user: SessionUser,
    mut conn: Db,
) -> Result<Template, Status> {
    let order = match Order::find_by_id(&mut *conn, id) {
        Ok(order) => order,
        Err(_) => return Err(Status::NotFound),
    };
    
    // 檢查訂單是否屬於當前使用者
    if order.user_id != user.id {
        return Err(Status::Forbidden);
    }
    
    let items = match OrderItem::with_products(&mut *conn, id) {
        Ok(items) => items,
        Err(_) => return Err(Status::InternalServerError),
    };
    
    Ok(Template::render("orders/show", context! {
        order,
        items,
    }))
}

#[post("/orders/create")]
pub async fn create(
    user: SessionUser,
    mut conn: Db,
) -> Result<Redirect, Status> {
    // 取得使用者的購物車
    let cart = match Cart::find_or_create(&mut *conn, user.id) {
        Ok(cart) => cart,
        Err(_) => return Err(Status::InternalServerError),
    };
    
    // 取得購物車項目
    let cart_items = match CartItem::with_products(&mut *conn, cart.id) {
        Ok(items) => items,
        Err(_) => return Err(Status::InternalServerError),
    };
    
    if cart_items.is_empty() {
        return Err(Status::BadRequest);
    }
    
    // 計算總金額
    let total = match CartItem::calculate_total(&mut *conn, cart.id) {
        Ok(total) => total,
        Err(_) => return Err(Status::InternalServerError),
    };
    
    // 建立訂單
    let new_order = NewOrder {
        user_id: user.id,
        total_amount: total.clone(),
        status: "pending".to_string(),
    };
    
    let order = match Order::create(&mut *conn, new_order) {
        Ok(order) => order,
        Err(_) => return Err(Status::InternalServerError),
    };
    
    // 建立訂單項目並更新庫存
    for item in cart_items {
        let new_order_item = NewOrderItem {
            order_id: order.id,
            product_id: item.product.id,
            quantity: item.cart_item.quantity,
            price: item.product.price.clone(),
        };
        
        if OrderItem::create(&mut *conn, new_order_item).is_err() {
            return Err(Status::InternalServerError);
        }
        
        // 更新產品庫存
        if Product::update_stock(&mut *conn, item.product.id, item.cart_item.quantity).is_err() {
            return Err(Status::InternalServerError);
        }
    }
    
    // 清空購物車
    if CartItem::clear_cart(&mut *conn, cart.id).is_err() {
        return Err(Status::InternalServerError);
    }
    
    Ok(Redirect::to(format!("/orders/{}", order.id)))
}

