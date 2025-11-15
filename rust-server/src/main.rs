// lib declarrations
use axum::{
        routing::get,
        Json,
        Router,
        extract::Query,
};
use serde::{Serialize, Deserialize};


#[derive(Debug, Deserialize)]
struct Params {
        name: Option<String>,
}

#[derive(Debug, Serialize)]
struct Out {
        message: String,
}

// Handler defintions
async fn compute_handler(Query(q): Query<Params>) -> Json<Out> {
        let name = q.name.unwrap_or("Anonymous".into());
        let result = format!("Hello, {}! (from Rust/Axum server)", name);

        Json(Out { message: result })
}

// main
#[tokio::main]
async fn main() {
        let app = Router::new()
                .route("/compute", get(compute_handler));


        let url = "0.0.0.0:3003";
        let listener = tokio::net::TcpListener::bind(url)
                .await
                .expect("Failed to bind port");


        axum::serve(listener, app)
                .await
                .expect("Server error");
}
