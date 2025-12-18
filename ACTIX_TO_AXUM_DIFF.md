# Actix Web to Axum Migration - Code Differences

## Overview
This document shows the key differences between Actix Web and Axum implementations.

---

## 1. Cargo.toml

### Actix Web
```toml
[package]
name = "actix-app"

[dependencies]
actix-web = "4.9.0"
env_logger = "0.11.6"
```

### Axum
```toml
[package]
name = "axum-app"

[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
tower = "0.4"
tower-http = { version = "0.5", features = ["trace"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```

**Key Changes:**
- Replaced `actix-web` with `axum`
- Added `tokio` runtime (required for Axum)
- Added `tower` and `tower-http` for middleware
- Replaced `env_logger` with `tracing` ecosystem

---

## 2. main.rs

### Actix Web
```rust
use actix_web::{middleware::Logger, web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    
    let shared_state = web::Data::new(AppState {
        person_collection: RwLock::new(person::create_person_collection()),
        greeting_text,
    });

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(shared_state.clone())
            .service(landing_page)
            .service(persons)
            // ... more services
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
```

### Axum
```rust
use axum::Router;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    let shared_state = Arc::new(AppState {
        person_collection: RwLock::new(person::create_person_collection()),
        greeting_text,
    });

    let app = Router::new()
        .merge(routes::create_routes())
        .layer(TraceLayer::new_for_http())
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();
    
    axum::serve(listener, app).await.unwrap();
}
```

**Key Changes:**
- `#[actix_web::main]` → `#[tokio::main]`
- `web::Data` → `Arc` for shared state
- `HttpServer::new()` → `tokio::net::TcpListener` + `axum::serve()`
- Services registered via `.service()` → Routes via `Router::new()`
- Middleware via `.wrap()` → `.layer()`

---

## 3. routes.rs

### Actix Web - Route Definition
```rust
use actix_web::{delete, get, post, put, web, HttpResponse};

#[get("/")]
async fn landing_page(data: web::Data<AppState>) -> AppResponse {
    Ok(HttpResponse::Ok().body(response_body))
}

#[get("/api/persons")]
async fn persons(data: web::Data<AppState>) -> AppResponse {
    let persons = data.person_collection.read()?;
    Ok(HttpResponse::Ok().json(persons.deref()))
}

#[get("/api/person/{id}")]
async fn single_person(path: web::Path<u32>, data: web::Data<AppState>) -> AppResponse {
    let id = path.into_inner();
    // ...
}

#[post("/api/person")]
async fn add_person(new_person: web::Json<Person>, data: web::Data<AppState>) -> AppResponse {
    let person = new_person.into_inner();
    // ...
}
```

### Axum - Route Definition
```rust
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};

pub fn create_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(landing_page))
        .route("/api/persons", get(persons))
        .route("/api/person/:id", get(single_person))
        .route("/api/person", post(add_person))
        .fallback(not_found_handler)
}

async fn landing_page(State(state): State<Arc<AppState>>) -> Html<String> {
    Html(response_body)
}

async fn persons(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Person>>, HttpAppError> {
    let persons = state.person_collection.read()?;
    Ok(Json(persons.clone()))
}

async fn single_person(
    Path(id): Path<u32>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Person>, HttpAppError> {
    // ...
}

async fn add_person(
    State(state): State<Arc<AppState>>,
    Json(person): Json<Person>,
) -> Result<StatusCode, HttpAppError> {
    // ...
}
```

**Key Changes:**
- Macro attributes `#[get("/path")]` → Function-based routing with `Router::new().route()`
- `web::Data<AppState>` → `State(state): State<Arc<AppState>>`
- `web::Path<T>` → `Path(value): Path<T>`
- `web::Json<T>` → `Json(value): Json<T>`
- `.into_inner()` not needed in Axum (destructured in parameters)
- `HttpResponse::Ok().json()` → `Json(data)`
- `HttpResponse::Created()` → `StatusCode::CREATED`
- Routes centralized in `create_routes()` function

---

## 4. errors.rs

### Actix Web
```rust
use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};

pub type AppResponse = Result<HttpResponse, HttpAppError>;

impl ResponseError for HttpAppError {
    fn status_code(&self) -> StatusCode {
        match self {
            HttpAppError::Conflict => StatusCode::CONFLICT,
            HttpAppError::NotFound => StatusCode::NOT_FOUND,
            HttpAppError::LockError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
}
```

### Axum
```rust
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

impl IntoResponse for HttpAppError {
    fn into_response(self) -> Response {
        let status = match self {
            HttpAppError::Conflict => StatusCode::CONFLICT,
            HttpAppError::NotFound => StatusCode::NOT_FOUND,
            HttpAppError::LockError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, Json(self.to_string())).into_response()
    }
}
```

**Key Changes:**
- `ResponseError` trait → `IntoResponse` trait
- `AppResponse` type alias removed (not needed)
- Simpler error response: tuple `(StatusCode, Json)` auto-converts
- No need for separate `status_code()` and `error_response()` methods

---

## 5. person.rs

### Actix Web
```rust
#[derive(Serialize, Deserialize)]
pub struct Person {
    pub id: u32,
    pub name: String,
    pub age: u8,
    pub date: NaiveDate,
}
```

### Axum
```rust
#[derive(Clone, Serialize, Deserialize)]
pub struct Person {
    pub id: u32,
    pub name: String,
    pub age: u8,
    pub date: NaiveDate,
}
```

**Key Changes:**
- Added `Clone` derive (Axum extractors often need cloneable data)

---

## Summary of Key Differences

| Aspect | Actix Web | Axum |
|--------|-----------|------|
| **Runtime** | Built-in actor system | Tokio async runtime |
| **Route Definition** | Macro-based `#[get("/path")]` | Function-based `Router::new().route()` |
| **State Sharing** | `web::Data<T>` | `State<Arc<T>>` |
| **Extractors** | `web::Path`, `web::Json` | `Path`, `Json` (destructured) |
| **Responses** | `HttpResponse::Ok().json()` | `Json(data)` or `StatusCode` |
| **Error Handling** | `ResponseError` trait | `IntoResponse` trait |
| **Middleware** | `.wrap()` | `.layer()` |
| **Logging** | `env_logger` + `Logger` | `tracing` + `TraceLayer` |
| **Philosophy** | Batteries-included framework | Modular, composable library |

---

## 6. Dockerfile

### Actix Web
```dockerfile
COPY --from=build /app/target/release/actix-app /app/server
```

### Axum
```dockerfile
COPY --from=build /app/target/release/axum-app /app/server
```

**Key Changes:**
- Binary name changed from `actix-app` to `axum-app` (matches Cargo.toml package name)

---

## Migration Checklist

- [x] Update dependencies in Cargo.toml
- [x] Change runtime from `actix_web::main` to `tokio::main`
- [x] Replace `HttpServer` with `axum::serve`
- [x] Convert macro-based routes to function-based routing
- [x] Update extractors: `web::Data` → `State`, `web::Path` → `Path`, etc.
- [x] Change response types: `HttpResponse` → `Json`/`StatusCode`/`Html`
- [x] Implement `IntoResponse` instead of `ResponseError`
- [x] Add `Clone` to shared data structures
- [x] Replace `env_logger` with `tracing`
- [x] Update middleware from `.wrap()` to `.layer()`
- [x] Update Dockerfile binary name from `actix-app` to `axum-app`
