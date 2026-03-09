use crate::collector::collect_sysinfo;

pub mod collector;
pub mod models;

#[tokio::main]
async fn main() {
    collect_sysinfo::run().await;
}
