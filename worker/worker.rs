#![deny(clippy::all, clippy::nursery)]
#![allow(clippy::future_not_send)]
#![deny(nonstandard_style, rust_2018_idioms)]

use worker::*;

use aws_region_nearby::find_region_nearby;

#[event(fetch)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
    Router::new()
        .get("/", |req, _ctx| {
            let (latitude, longitude) = req.cf().coordinates().unwrap();
            let region = find_region_nearby(latitude, longitude);
            Response::from_json(&region)
        })
        .run(req, env)
        .await
}
