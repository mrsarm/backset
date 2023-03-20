#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub max_connections: u32,
    // Add more config here
}

impl Config {
    pub fn init() -> Config {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let max_connections = std::env::var("MAX_CONNECTIONS").unwrap_or(String::from("10"));
        let max_connections = max_connections
            .parse::<u32>()
            .expect(&("MAX_CONNECTIONS invalid integer ".to_owned() + &max_connections));
        Config {
            database_url,
            max_connections,
        }
    }
}
