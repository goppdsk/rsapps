use graphql_client::GraphQLQuery;
use serde_json::json;
use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

type DateTimeUtc = String;

#[derive(GraphQLQuery)]
#[graphql(schema_path = "gql/schema.json", query_path = "gql/all_todos.graphql")]
pub struct AllTodos;

#[derive(GraphQLQuery)]
#[graphql(schema_path = "gql/schema.json", query_path = "gql/new_todo.graphql")]
pub struct CreateNewTodo;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "gql/schema.json",
    query_path = "gql/complete_todo.graphql"
)]
pub struct ToggleComplete;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "gql/schema.json",
    query_path = "gql/complete_all_todos.graphql"
)]
pub struct ToggleAllComplete;

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

pub async fn fetch_all_todos() -> Result<Vec<all_todos::AllTodosTodos>, FetchError> {
    let request_body = AllTodos::build_query(all_todos::Variables {});
    let resp =
        request::<all_todos::Variables>("http://localhost:8081/graphql", request_body).await?;

    match resp.into_serde::<graphql_client::Response<all_todos::ResponseData>>() {
        Ok(data) => match data.data {
            Some(data) => Ok(data.todos),
            None => Ok(vec![]),
        },
        Err(_) => Err(FetchError {
            err: JsValue::from_str("failed to fecth all todos"),
        }),
    }
}

pub async fn create_todo(
    body: String,
) -> Result<create_new_todo::CreateNewTodoCreateTodo, FetchError> {
    let request_body = CreateNewTodo::build_query(create_new_todo::Variables {
        todo: create_new_todo::NewTodo { body },
    });
    let resp = request::<create_new_todo::Variables>("http://localhost:8081/graphql", request_body)
        .await?;

    match resp.into_serde::<graphql_client::Response<create_new_todo::ResponseData>>() {
        Ok(data) => match data.data {
            Some(data) => Ok(data.create_todo),
            None => Err(FetchError {
                err: JsValue::from_str("failed to create new todo"),
            }),
        },
        Err(_) => Err(FetchError {
            err: JsValue::from_str("failed to create new todo"),
        }),
    }
}

pub async fn toggle_complete_todo(id: i64) -> Result<bool, FetchError> {
    let request_body = ToggleComplete::build_query(toggle_complete::Variables { id });
    let resp = request::<toggle_complete::Variables>("http://localhost:8081/graphql", request_body)
        .await?;

    match resp.into_serde::<graphql_client::Response<toggle_complete::ResponseData>>() {
        Ok(data) => match data.data {
            Some(data) => Ok(data.toggle_complete),
            None => Err(FetchError {
                err: JsValue::from_str(format!("failed to complete todo, id: {}", id).as_str()),
            }),
        },
        Err(_) => Err(FetchError {
            err: JsValue::from_str(format!("failed to complete todo, id: {}", id).as_str()),
        }),
    }
}

pub async fn toggle_complete_all_todos() -> Result<bool, FetchError> {
    let request_body = ToggleAllComplete::build_query(toggle_all_complete::Variables {});
    let resp =
        request::<toggle_all_complete::Variables>("http://localhost:8081/graphql", request_body)
            .await?;

    match resp.into_serde::<graphql_client::Response<toggle_all_complete::ResponseData>>() {
        Ok(data) => match data.data {
            Some(data) => Ok(data.toggle_all_complete),
            None => Err(FetchError {
                err: JsValue::from_str("failed to complete all todos"),
            }),
        },
        Err(_) => Err(FetchError {
            err: JsValue::from_str("failed to complete all todos"),
        }),
    }
}

pub async fn request<V: serde::Serialize>(
    url: &str,
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
    let request = Request::new_with_str_and_init(url, &opts)?;

    let window = yew::utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();

    Ok(JsFuture::from(resp.json()?).await?)
}
