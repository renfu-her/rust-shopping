use rocket::figment::Figment;

pub struct Config;

impl Config {
    pub fn get_database_url() -> String {
        std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "mysql://root:password@localhost/shopping".to_string())
    }
}

