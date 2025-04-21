use std::{
	collections::HashMap, sync::{Arc, RwLock}
};

use axum::{
    Router,
    extract::{Path, State, Query},
    http::StatusCode,
    routing::{get, post},
	response::Redirect,
	Json
};
use serde::Deserialize;

use crate::data::{Board, Thread, Post};

type SharedState = Arc<RwLock<AppState>>;

#[derive(Debug, Deserialize)]
pub struct AppState {
    pub boards: Vec<Board>,
}

pub fn get_router<'a>() -> Router<SharedState>
{
    Router::new()
		.route("/{board_id}", get(get_board_threads))
		.route("/{board_id}/new", post(new_thread))
		.route("/{board_id}/{thread_id}", get(get_thread))
		.route("/{board_id}/{thread_id}/new", post(new_post))
}


pub async fn get_board_threads(
    Path(board_id): Path<usize>,
    State(state): State<SharedState>
) -> Result<Json<Board>, StatusCode> {
	let state = state.read().unwrap();
	if board_id >= state.boards.len() {
		return Err(StatusCode::NOT_FOUND);
	}

	let board = &state.boards[board_id];
    Ok(Json(Board{
		name: board.name.clone(),
		threads: board.threads.iter()
			.map(|thread| {
				Thread {
					title: thread.title.clone(),
					posts: vec![thread.posts[0].clone()]
				}
			})
			.collect()
	}))
}

pub async fn new_thread(
    Path(board_id): Path<usize>,
	State(state): State<SharedState>,
	Json(payload): Json<Thread>
) -> Result<Redirect, StatusCode> {
	// TODO: Find a better error code
	if payload.posts.len() != 1 {
		return Err(StatusCode::METHOD_NOT_ALLOWED);
	}

	let mut state = state.write().unwrap();
	if board_id >= state.boards.len() {
		return Err(StatusCode::NOT_FOUND);
	}

	let threads = &mut state.boards[board_id].threads;
	let thread_id = threads.len();
	threads.push(payload);
	
	Ok(Redirect::permanent(format!("/{}/{}", board_id, thread_id).as_str()))
}

pub async fn get_thread(
	Path((board_id, thread_id)): Path<(usize, usize)>,
	State(state): State<SharedState>,
	Query(params): Query<HashMap<String, usize>>	
) -> Result<Json<Thread>, StatusCode> {
	let state = state.read().unwrap();
	if board_id >= state.boards.len() {
		return Err(StatusCode::NOT_FOUND);
	}

	let board = &state.boards[board_id];
	if thread_id >= board.threads.len() {
		return Err(StatusCode::NOT_FOUND);
	}
	let thread = &board.threads[thread_id];

	let offset = *params.get("offset").unwrap_or(&0);
	let limit = params.get("limit");
	
	Ok(Json(
		match limit {
		Some(limit) => Thread {
			title: thread.title.clone(),
			posts: Vec::from(&thread.posts[offset .. offset+limit])
		},
		None => Thread {
			title: thread.title.clone(),
			posts: Vec::from(&thread.posts[offset ..])
		}
		}
	))
}

pub async fn new_post(
	Path((board_id, thread_id)): Path<(usize, usize)>,
	State(state): State<SharedState>,
	Json(payload): Json<Post>
) -> Result<(), StatusCode> {
	let mut state = state.write().unwrap();
	if board_id >= state.boards.len() {
		return Err(StatusCode::NOT_FOUND);
	}

	let threads = &mut state.boards[board_id].threads;
	if thread_id >= threads.len() {
		return Err(StatusCode::NOT_FOUND);
	}

	let posts = &mut threads[thread_id].posts;
	posts.push(payload);
	
	Ok(())
}
