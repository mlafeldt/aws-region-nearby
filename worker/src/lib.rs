use worker::*;

use aws_region_nearby::find_region_nearby;

#[event(fetch)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
    Router::new()
        .get_async("/", |req, _ctx| async move {
            let (latitude, longitude) = req.cf().coordinates().unwrap();
            let region = find_region_nearby(latitude as f64, longitude as f64);
            Response::ok(region)
        })
        .run(req, env)
        .await
}
