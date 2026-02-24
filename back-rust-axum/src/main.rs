use axum::extract::{Path, Query, rejection::QueryRejection};
use axum::http::{HeaderMap, StatusCode};
use axum::{Router, extract::State, middleware::map_response, response::Response, routing};
use std::ops::Not;
use std::sync::Arc;
use std::sync::atomic::AtomicIsize;
use std::sync::atomic::Ordering::Relaxed;
use tokio;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let app = Router::new()
        .route("/", routing::delete(api_delete))
        .route("/", routing::get(api_get))
        .route("/", routing::post(api_increment))
        .route("/set-by-path/{id}", routing::put(api_set_by_path))
        .route("/set-by-path/{id}/", routing::put(api_set_by_path_fail))
        .route(
            "/set-by-path",
            routing::put(api_set_by_path_fail).fallback(method_not_allowed),
        )
        .route(
            "/set-by-query",
            routing::put(api_set_by_query).fallback(method_not_allowed),
        )
        .route(
            "/set-by-header",
            routing::put(api_set_by_header).fallback(method_not_allowed),
        )
        .route(
            "/set-by-body",
            routing::put(api_set_by_body).fallback(method_not_allowed),
        )
        .fallback(fallback)
        .layer(map_response(set_header_json))
        .with_state(std::sync::Arc::new(AtomicIsize::new(1)));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn method_not_allowed() -> (StatusCode, &'static str) {
    (
        StatusCode::METHOD_NOT_ALLOWED,
        "\"405 Method not allowed\"\r\n",
    )
}

async fn fallback() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "\"404 Path not found\"\r\n")
}

async fn set_header_json<B>(mut response: Response<B>) -> Response<B> {
    response
        .headers_mut()
        .insert("Content-Type", "application/json".parse().unwrap());
    response
}

async fn api_delete(State(state): State<Arc<AtomicIsize>>) -> &'static str {
    state.store(0, Relaxed);
    "0"
}

async fn api_get(State(state): State<Arc<AtomicIsize>>) -> String {
    format!("{}", state.load(Relaxed))
}

async fn api_increment(State(state): State<Arc<AtomicIsize>>) -> String {
    format!("{}", state.fetch_add(1, Relaxed) + 1)
}

async fn api_set_by_path(State(state): State<Arc<AtomicIsize>>, Path(n): Path<isize>) -> String {
    state.store(n, Relaxed);
    format!("{}", n)
}
async fn api_set_by_path_fail() -> (StatusCode, &'static str) {
    (
        StatusCode::BAD_REQUEST,
        "\"400 Need a path as /set-by-path/:int\"\r\n",
    )
}

#[derive(serde::Deserialize)]
struct ApiSetByQuery {
    nb: isize,
}
#[axum::debug_handler]
pub(crate) async fn api_set_by_query(
    State(state): State<Arc<AtomicIsize>>,
    data: Result<Query<ApiSetByQuery>, QueryRejection>,
) -> (StatusCode, String) {
    match data {
        Err(_) => (
            StatusCode::BAD_REQUEST,
            String::from("\"400 Need integer in query with name nb\"\r\n"),
        ),
        Ok(Query(ApiSetByQuery { nb })) => {
            state.store(nb, Relaxed);
            (StatusCode::OK, format!("{}", nb))
        }
    }
}

async fn api_set_by_header(
    State(state): State<Arc<AtomicIsize>>,
    headers: HeaderMap,
) -> (StatusCode, String) {
    let nb = headers
        .get("x-nb")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.is_empty().not().then_some(value))
        .and_then(|s| s.parse::<isize>().ok());

    match nb {
        None => (
            StatusCode::BAD_REQUEST,
            String::from("\"400 Need integer in header with name x-nb\"\r\n"),
        ),
        Some(nb) => {
            state.store(nb, Relaxed);
            (StatusCode::OK, nb.to_string())
        }
    }
}

async fn api_set_by_body(
    State(state): State<Arc<AtomicIsize>>,
    body: String,
) -> (StatusCode, String) {
    match body.parse::<isize>() {
        Err(_) => (
            StatusCode::BAD_REQUEST,
            String::from("\"400 Need integer in the body\"\r\n"),
        ),
        Ok(nb) => {
            state.store(nb, Relaxed);
            (StatusCode::OK, nb.to_string())
        }
    }
}
