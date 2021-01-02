use serde_json::json;
use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    pub err: JsValue,
}
impl Display for FetchError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.err, f)
    }
}
impl Error for FetchError {}

pub static GRAPHQL_ENDPOINT: &str = std::env!("GRAPHQL_ENDPOINT");

pub async fn request<V: serde::Serialize>(
    query: graphql_client::QueryBody<V>,
) -> Result<JsValue, FetchError> {
    let json_body = json!(query);
    let headers = match JsValue::from_serde(&json!({
        "Content-Type": "application/json"
    })) {
        Ok(headers) => headers,
        Err(_) => JsValue::NULL,
    };
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
    opts.body(Some(JsValue::from_str(json_body.to_string().as_str())).as_ref());
    opts.headers(&headers);
    let request = Request::new_with_str_and_init(GRAPHQL_ENDPOINT, &opts)?;

    let window = yew::utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();

    Ok(JsFuture::from(resp.json()?).await?)
}
