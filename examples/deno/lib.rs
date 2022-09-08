use wasm_bindgen::prelude::*;
use web_sys::{Request, Response};

use aws_region_nearby::*;

#[wasm_bindgen]
pub async fn handler(_req: Request) -> Result<Response, JsValue> {
    let deno_region: DenoRegion = env_var("DENO_REGION").unwrap().parse().unwrap();
    let aws_region: AwsRegion = deno_region.into();

    Response::new_with_opt_str(Some(&format!("Deno: {deno_region}\nAWS: {aws_region}")))
}

#[wasm_bindgen(inline_js = "export const env_var = (name) => Deno.env.get(name)")]
extern "C" {
    pub fn env_var(key: &str) -> Option<String>;
}
