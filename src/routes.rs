use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Html,
    routing::{delete, get, post, put},
    Json, Router,
};
use std::sync::{Arc, RwLock};
use crate::errors::HttpAppError;
use crate::person::Person;

pub struct AppState {
    pub person_collection: RwLock<Vec<Person>>,
    pub greeting_text: String,
}

pub fn create_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(landing_page))
        .route("/health", get(health))
        .route("/api/persons", get(persons))
        .route("/api/person/:id", get(single_person))
        .route("/api/person", post(add_person))
        .route("/api/person", put(update_person))
        .route("/api/person/:id", delete(delete_person))
        .fallback(not_found_handler)
}

async fn landing_page(State(state): State<Arc<AppState>>) -> Html<String> {
    use chrono::Utc;
    let current_time = Utc::now().to_rfc3339();
    let response_body = format!("Rust-Axum {} <br> Current UTC time: {}", state.greeting_text, current_time);
    Html(response_body)
}

async fn not_found_handler() -> Html<&'static str> {
    Html("Oops! The page you are looking for does not exist.")
}

async fn health() -> &'static str {
    "OK"
}

async fn persons(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Person>>, HttpAppError> {
    let persons = state.person_collection.read()?;
    Ok(Json(persons.clone()))
}

async fn single_person(
    Path(id): Path<u32>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Person>, HttpAppError> {
    let persons_guard = state.person_collection.read()?;
    let filtered = persons_guard.iter().find(|t| t.id == id);
    match filtered {
        Some(filtered) => Ok(Json(filtered.clone())),
        None => Err(HttpAppError::NotFound),
    }
}

async fn add_person(
    State(state): State<Arc<AppState>>,
    Json(person): Json<Person>,
) -> Result<StatusCode, HttpAppError> {
    let mut persons_guard = state.person_collection.write()?;
    let filtered = persons_guard.iter().any(|t| t.id == person.id);
    if !filtered {
        persons_guard.push(person);
        Ok(StatusCode::CREATED)
    } else {
        Err(HttpAppError::Conflict)
    }
}

async fn update_person(
    State(state): State<Arc<AppState>>,
    Json(person): Json<Person>,
) -> Result<StatusCode, HttpAppError> {
    let mut persons_guard = state.person_collection.write()?;
    let filtered = persons_guard.iter_mut().find(|t| t.id == person.id);
    match filtered {
        Some(p) => {
            p.age = person.age;
            p.date = person.date;
            p.name = person.name;
            Ok(StatusCode::NO_CONTENT)
        }
        None => Err(HttpAppError::NotFound),
    }
}

async fn delete_person(
    Path(id): Path<u32>,
    State(state): State<Arc<AppState>>,
) -> Result<StatusCode, HttpAppError> {
    let mut persons_guard = state.person_collection.write()?;
    let index = persons_guard.iter().position(|t| t.id == id);
    match index {
        Some(index) => {
            persons_guard.remove(index);
            Ok(StatusCode::NO_CONTENT)
        }
        None => Err(HttpAppError::NotFound),
    }
}
