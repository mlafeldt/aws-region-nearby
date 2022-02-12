use worker::*;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    Router::new()
        .get("/", |req, _ctx| {
            let (latitude, longitude) = req.cf().coordinates().unwrap();
            let region = aws_region_nearby::find_region(latitude, longitude);
            Response::ok(region.name())
        })
        .run(req, env)
        .await
}
