use gloo_console::log;
use tiny_dynamo::{reqwest_transport, Credentials, Table, DB};
use wasm_bindgen::prelude::*;
use web_sys::{Request, Response};

use aws_region_nearby::{AwsRegion, DenoRegion};

#[wasm_bindgen]
pub async fn handler(req: Request) -> Result<Response, JsValue> {
    console_error_panic_hook::set_once();

    let replica_regions: Vec<AwsRegion> = env_var("REPLICA_REGIONS")
        .unwrap()
        .split(',')
        .map(|r| r.parse().unwrap())
        .collect();
    let deno_region: DenoRegion = env_var("DENO_REGION").unwrap().parse().unwrap();
    let aws_region = deno_region.find_region_from_list(&replica_regions);

    let db = DB::new(
        Credentials::new(
            env_var("AWS_ACCESS_KEY_ID").unwrap(),
            env_var("AWS_SECRET_ACCESS_KEY").unwrap(),
        ),
        Table::new(
            env_var("TABLE_NAME").unwrap(),
            "key",
            "value",
            aws_region.to_string().parse().unwrap(),
            None,
        ),
        reqwest_transport::Reqwest::new(),
    );

    let now = js_sys::Date::new_0().get_time();
    let ip = req.headers().get("X-Forwarded-For")?.unwrap();

    let (key, value) = (format!("DENO#{now}"), format!("{ip} {deno_region} -> {aws_region}"));
    log!(format!("{key} = {value}"));

    db.set(&key, value).await.map_err(|e| format!("PutItem error: {e}"))?;
    let res = db.get(&key).await.map_err(|e| format!("GetItem error: {e}"))?;

    Response::new_with_opt_str(res.as_deref())
}

#[wasm_bindgen(inline_js = "export const env_var = (name) => Deno.env.get(name)")]
extern "C" {
    pub fn env_var(key: &str) -> Option<String>;
}
