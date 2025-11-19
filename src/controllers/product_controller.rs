use rocket::form::Form;
use rocket::response::Redirect;
use rocket::http::Status;
use rocket_dyn_templates::{Template, context};
use rocket::fs::{TempFile, relative};
use diesel::prelude::*;
use bigdecimal::BigDecimal;
use crate::database::Db;
use crate::models::product::{Product, NewProduct, UpdateProduct};
use crate::models::category::Category;
use crate::middleware::auth::SessionUser;
use crate::utils::pagination::Pagination;
use crate::utils::file_upload::save_uploaded_file;

#[get("/products?<page>&<search>&<category_id>&<min_price>&<max_price>")]
pub async fn index(
    page: Option<i64>,
    search: Option<String>,
    category_id: Option<i32>,
    min_price: Option<f64>,
    max_price: Option<f64>,
    mut conn: Db,
    user: Option<SessionUser>,
) -> Template {
    let current_page = page.unwrap_or(1);
    let per_page = 12;
    
    let mut query = crate::schema::products::table.into_boxed();
    
    if let Some(search_str) = &search {
        let pattern = format!("%{}%", search_str);
        query = query.filter(
            crate::schema::products::name.like(&pattern)
                .or(crate::schema::products::description.like(&pattern))
        );
    }
    
    if let Some(cat_id) = category_id {
        query = query.filter(crate::schema::products::category_id.eq(cat_id));
    }
    
    if let Some(min) = min_price {
        query = query.filter(crate::schema::products::price.ge(BigDecimal::from(min)));
    }
    
    if let Some(max) = max_price {
        query = query.filter(crate::schema::products::price.le(BigDecimal::from(max)));
    }
    
    let total: i64 = query
        .count()
        .get_result(&mut *conn)
        .unwrap_or(0);
    
    let pagination = Pagination::new(current_page, per_page, total);
    
    let products: Vec<Product> = query
        .limit(per_page)
        .offset(pagination.offset())
        .load(&mut *conn)
        .unwrap_or_default();
    
    let categories = Category::all(&mut *conn).unwrap_or_default();
    
    Template::render("products/index", context! {
        products,
        categories,
        pagination,
        search: search.unwrap_or_default(),
        category_id,
        min_price,
        max_price,
        user,
    })
}

#[get("/products/<id>")]
pub async fn show(id: i32, mut conn: Db, user: Option<SessionUser>) -> Result<Template, Status> {
    match Product::find_with_category(&mut *conn, id) {
        Ok(product_with_category) => {
            Ok(Template::render("products/show", context! {
                product: product_with_category.product,
                category: product_with_category.category,
                user,
            }))
        }
        Err(_) => Err(Status::NotFound),
    }
}

#[get("/products/create")]
pub async fn create_page(mut conn: Db, _user: SessionUser) -> Template {
    let categories = Category::all(&mut *conn).unwrap_or_default();
    Template::render("products/create", context! {
        categories,
    })
}

#[post("/products", data = "<form>")]
pub async fn create(
    form: Form<ProductForm>,
    _user: SessionUser,
    mut conn: Db,
) -> Result<Redirect, Template> {
    let product_data = form.into_inner();
    
    let new_product = NewProduct {
        category_id: product_data.category_id,
        name: product_data.name,
        description: Some(product_data.description),
        price: BigDecimal::from(product_data.price),
        stock: product_data.stock,
        image_url: None,
    };
    
    match Product::create(&mut *conn, new_product) {
        Ok(_) => Ok(Redirect::to("/products")),
        Err(_) => {
            let categories = Category::all(&mut *conn).unwrap_or_default();
            Err(Template::render("products/create", context! {
                categories,
                error: "Failed to create product",
            }))
        }
    }
}

#[get("/products/<id>/edit")]
pub async fn edit_page(id: i32, mut conn: Db, _user: SessionUser) -> Result<Template, Status> {
    match Product::find_by_id(&mut *conn, id) {
        Ok(product) => {
            let categories = Category::all(&mut *conn).unwrap_or_default();
            Ok(Template::render("products/edit", context! {
                product,
                categories,
            }))
        }
        Err(_) => Err(Status::NotFound),
    }
}

#[post("/products/<id>", data = "<form>")]
pub async fn update(
    id: i32,
    form: Form<ProductForm>,
    _user: SessionUser,
    mut conn: Db,
) -> Result<Redirect, Status> {
    let product_data = form.into_inner();
    
    let update_data = UpdateProduct {
        category_id: Some(product_data.category_id),
        name: Some(product_data.name),
        description: Some(Some(product_data.description)),
        price: Some(BigDecimal::from(product_data.price)),
        stock: Some(product_data.stock),
        image_url: None,
    };
    
    match Product::update(&mut *conn, id, update_data) {
        Ok(_) => Ok(Redirect::to(format!("/products/{}", id))),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/products/<id>/delete")]
pub async fn delete(id: i32, _user: SessionUser, mut conn: Db) -> Result<Redirect, Status> {
    match Product::delete(&mut *conn, id) {
        Ok(_) => Ok(Redirect::to("/products")),
        Err(_) => Err(Status::NotFound),
    }
}

#[post("/products/<id>/upload-image", data = "<file>")]
pub async fn upload_image(
    id: i32,
    file: TempFile<'_>,
    _user: SessionUser,
    mut conn: Db,
) -> Result<Redirect, Status> {
    let upload_dir = relative!("static/images");
    
    match save_uploaded_file(file, upload_dir).await {
        Ok(image_url) => {
            let update_data = UpdateProduct {
                category_id: None,
                name: None,
                description: None,
                price: None,
                stock: None,
                image_url: Some(Some(image_url)),
            };
            
            match Product::update(&mut *conn, id, update_data) {
                Ok(_) => Ok(Redirect::to(format!("/products/{}", id))),
                Err(_) => Err(Status::InternalServerError),
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[derive(FromForm)]
pub struct ProductForm {
    pub category_id: i32,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub stock: i32,
}

