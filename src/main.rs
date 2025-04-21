use std::sync::{Arc, RwLock};

use axum::Router;
use config::Config;
use tower_http::services::{ServeDir, ServeFile};

mod api;
mod data;

use api::AppState;

#[tokio::main]
async fn main() {
    let settings = Config::builder()
        .add_source(config::File::with_name("./boards"))
        .build()
        .unwrap();

    let app_state = settings.try_deserialize::<AppState>().unwrap();
	dbg!(&app_state);

	let app = Router::new()
		.nest("/api", api::get_router())
		.nest_service("/thread", ServeFile::new("static/thread.html"))
		.fallback_service(ServeDir::new("static"))
		.with_state(Arc::new(RwLock::new(app_state)))
	;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
