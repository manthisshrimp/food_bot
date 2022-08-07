#[macro_use]
extern crate diesel;
extern crate dotenv;

mod database;
mod schema;
mod models;
mod telegram_bot;

pub use telegram_bot::run;
