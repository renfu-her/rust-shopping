use rocket_db_pools::{Database, Connection};
use diesel::mysql::MysqlConnection;

#[derive(Database)]
#[database("shopping_db")]
pub struct DbConn(diesel::r2d2::ConnectionManager<MysqlConnection>);

pub type Db = Connection<DbConn>;

