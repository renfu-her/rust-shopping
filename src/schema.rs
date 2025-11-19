// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    categories (id) {
        id -> Integer,
        name -> Varchar,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    products (id) {
        id -> Integer,
        category_id -> Integer,
        name -> Varchar,
        description -> Nullable<Text>,
        price -> Decimal,
        stock -> Integer,
        image_url -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    carts (id) {
        id -> Integer,
        user_id -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    cart_items (id) {
        id -> Integer,
        cart_id -> Integer,
        product_id -> Integer,
        quantity -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    orders (id) {
        id -> Integer,
        user_id -> Integer,
        total_amount -> Decimal,
        status -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    order_items (id) {
        id -> Integer,
        order_id -> Integer,
        product_id -> Integer,
        quantity -> Integer,
        price -> Decimal,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(products -> categories (category_id));
diesel::joinable!(carts -> users (user_id));
diesel::joinable!(cart_items -> carts (cart_id));
diesel::joinable!(cart_items -> products (product_id));
diesel::joinable!(orders -> users (user_id));
diesel::joinable!(order_items -> orders (order_id));
diesel::joinable!(order_items -> products (product_id));

diesel::allow_tables_to_appear_in_same_query!(
    users,
    categories,
    products,
    carts,
    cart_items,
    orders,
    order_items,
);

