use tiny_dynamo::{reqwest_transport, Credentials, Table, DB};
use worker::*;

use aws_region_nearby::{find_region_from_list, AwsRegion};

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    if req.path() != "/" {
        return Response::error("Not found", 404);
    }

    let replica_regions: Vec<AwsRegion> = env
        .secret("REPLICA_REGIONS")
        .unwrap()
        .to_string()
        .split(',')
        .map(|r| r.parse().unwrap())
        .collect();
    let (latitude, longitude) = req.cf().coordinates().unwrap();
    let aws_region = find_region_from_list(latitude, longitude, &replica_regions);

    let db = DB::new(
        Credentials::new(
            env.secret("AWS_ACCESS_KEY_ID").unwrap().to_string(),
            env.secret("AWS_SECRET_ACCESS_KEY").unwrap().to_string(),
        ),
        Table::new(
            env.secret("TABLE_NAME").unwrap().to_string(),
            "key",
            "value",
            aws_region.to_string().parse().unwrap(),
            None,
        ),
        reqwest_transport::Reqwest::new(),
    );

    let now = js_sys::Date::new_0().get_time();
    let ip = &req.headers().get("x-real-ip").ok().flatten().unwrap();

    let (key, value) = (
        format!("WORKER#{now}"),
        format!("{ip} ({latitude} {longitude}) -> {aws_region}"),
    );
    console_log!("{key} = {value}");

    db.set(&key, value).await.map_err(|e| format!("PutItem error: {e}"))?;
    let res = db.get(&key).await.map_err(|e| format!("GetItem error: {e}"))?;

    Response::ok(res.unwrap())
}
