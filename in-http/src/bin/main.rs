use warp::{Filter, self};
use in_data::{models::user::*, datatypes::graph::*};

#[tokio::main]
async fn main() {
    let root = warp::path::end().map(|| "EEE");
    let o = warp::path!("user" / u32).map(|a| format!("Oh hi {}", a));
    let api = root.or(o);
    warp::serve(api).run(([0, 0, 0, 0], 3030)).await;
    println!("Hello, world!");

}
