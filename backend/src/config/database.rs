use diesel::r2d2::{ Pool, ConnectionManager, PooledConnection };
use diesel::pg::PgConnection;
use crate::config::Settings;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;


pub fn create_pool(settings: &Settings) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(&settings.database_url);
    
    Pool::builder()
        .max_size(settings.max_pool_size)
        .min_idle(Some(settings.min_idle_size))
        .test_on_check_out(true)
        .build(manager)
        .expect("Failed to create connection pool")
}