#[macro_use] extern crate rocket;

use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use shopping::database::DbConn;
use shopping::controllers::{
    auth_controller, product_controller, category_controller,
    cart_controller, order_controller
};

#[launch]
fn rocket() -> _ {
    // 載入 .env 檔案
    dotenvy::dotenv().ok();
    
    // 如果設定了 DATABASE_URL，將其轉換為 Rocket 需要的環境變數格式
    if let Ok(database_url) = std::env::var("DATABASE_URL") {
        std::env::set_var("ROCKET_DATABASES_shopping_db_url", database_url);
    }
    
    rocket::build()
        .attach(DbConn::init())
        .attach(Template::fairing())
        .mount("/", routes![
            // Auth routes
            auth_controller::login_page,
            auth_controller::login,
            auth_controller::register_page,
            auth_controller::register,
            auth_controller::logout,
            // Product routes
            product_controller::index,
            product_controller::show,
            product_controller::create_page,
            product_controller::create,
            product_controller::edit_page,
            product_controller::update,
            product_controller::delete,
            product_controller::upload_image,
            // Category routes
            category_controller::index,
            category_controller::create_page,
            category_controller::create,
            category_controller::edit_page,
            category_controller::update,
            category_controller::delete,
            // Cart routes
            cart_controller::index,
            cart_controller::add,
            cart_controller::update_item,
            cart_controller::delete_item,
            // Order routes
            order_controller::index,
            order_controller::show,
            order_controller::create,
        ])
        .mount("/static", FileServer::from(relative!("static")))
}
