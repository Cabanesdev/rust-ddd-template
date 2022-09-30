pub mod app;
pub mod contexts;

use app::server::infrastructure::server::create_and_start;

fn main() {
    println!("Hello, world!");
    create_and_start().unwrap();
}
