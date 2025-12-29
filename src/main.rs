mod to_do_list;
mod to_do_item;
mod input_handler;

use input_handler::input_handler;

#[tokio::main]
async fn main() {
    input_handler().await;
}