#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret_key: String,
    pub jwt_expires: i64,
    pub port: u16,
}

impl Config {
    pub fn init() -> Config {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE URL not config!");
        let jwt_secret_key = std::env::var("JWT_SECRET_KEY").expect("JWT SECRET KEY not config!");
        let jwt_expires = std::env::var("JWT_EXPIRES").expect("JWT EXPIRES not config!");
        
        Config {
            database_url,
            jwt_secret_key,
            jwt_expires: jwt_expires.parse::<i64>().unwrap(),
            port: 3333,
        }
    }
}