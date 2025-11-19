use rocket_db_pools::{Database, Connection};
use diesel::mysql::MysqlConnection;

#[derive(Database)]
#[database("shopping_db")]
pub struct DbConn(diesel::MysqlConnection);

pub type Db = Connection<DbConn>;

impl DbConn {
    pub fn get_url() -> String {
        std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "mysql://root:password@localhost/shopping".to_string())
    }
}

