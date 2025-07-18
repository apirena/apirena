// Axum Basic Routes Test
// Tests: Basic HTTP methods with Axum framework

use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::net::TcpListener;

#[derive(Serialize, Deserialize)]
struct User {
    id: Option<u32>,
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Post {
    id: Option<u32>,
    title: String,
    user_id: u32,
}

#[derive(Deserialize)]
struct SearchQuery {
    q: Option<String>,
    limit: Option<u32>,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        // User routes
        .route("/users", get(get_users))
        .route("/users", post(create_user))
        .route("/users/:id", get(get_user))
        .route("/users/:id", put(update_user))
        .route("/users/:id", delete(delete_user))
        // Post routes
        .route("/api/posts", get(get_posts))
        .route("/api/posts", post(create_post))
        .route("/api/posts/:id", get(get_post))
        // Health and admin
        .route("/health", get(health_check))
        .route("/admin/stats", get(admin_stats))
        // Search
        .route("/search", get(search));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn get_users() -> Json<Vec<User>> {
    Json(vec![
        User {
            id: Some(1),
            name: "John Doe".to_string(),
        },
        User {
            id: Some(2),
            name: "Jane Smith".to_string(),
        },
    ])
}

async fn create_user(Json(mut user): Json<User>) -> (StatusCode, Json<User>) {
    user.id = Some(123);
    (StatusCode::CREATED, Json(user))
}

async fn get_user(Path(id): Path<u32>) -> Json<User> {
    Json(User {
        id: Some(id),
        name: format!("User {}", id),
    })
}

async fn update_user(Path(id): Path<u32>, Json(mut user): Json<User>) -> Json<User> {
    user.id = Some(id);
    Json(user)
}

async fn delete_user(Path(_id): Path<u32>) -> StatusCode {
    StatusCode::NO_CONTENT
}

async fn get_posts() -> Json<Vec<Post>> {
    Json(vec![])
}

async fn create_post(Json(mut post): Json<Post>) -> (StatusCode, Json<Post>) {
    post.id = Some(1);
    (StatusCode::CREATED, Json(post))
}

async fn get_post(Path(id): Path<u32>) -> Json<Post> {
    Json(Post {
        id: Some(id),
        title: format!("Post {}", id),
        user_id: 1,
    })
}

async fn health_check() -> Json<HashMap<String, String>> {
    let mut response = HashMap::new();
    response.insert("status".to_string(), "ok".to_string());
    response.insert("service".to_string(), "axum-app".to_string());
    Json(response)
}

async fn admin_stats() -> Json<HashMap<String, u32>> {
    let mut stats = HashMap::new();
    stats.insert("users".to_string(), 100);
    stats.insert("posts".to_string(), 50);
    Json(stats)
}

async fn search(Query(params): Query<SearchQuery>) -> Json<HashMap<String, String>> {
    let mut response = HashMap::new();
    response.insert("query".to_string(), params.q.unwrap_or_default());
    response.insert("limit".to_string(), params.limit.unwrap_or(10).to_string());
    Json(response)
}
