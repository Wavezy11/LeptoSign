use axum::{
    extract::{State, Query},
    routing::{post, get, put, delete},
    Json, Router, Server,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::net::SocketAddr;
use tower_http::cors::{CorsLayer, Any};

// Subscriber struct
#[derive(Deserialize, Serialize, Debug)]
struct Subscriber {
    id: Option<i32>,
    email: Option<String>,
    surname: Option<String>,
    lastname: Option<String>,
    address: Option<String>,
    city: Option<String>,
    postal_code: Option<String>,
    phone_number: Option<String>,
}

// Alleen id nodig voor delete
#[derive(Deserialize, Debug)]
struct DeleteParams {
    id: i32,
}

// Insert new subscriber
async fn subscribe(
    State(pool): State<PgPool>,
    Json(sub): Json<Subscriber>,
) -> &'static str {
    let _ = sqlx::query(
        "INSERT INTO subscription (email, surname, lastname, address, city, postal_code, phone_number) \
         VALUES ($1,$2,$3,$4,$5,$6,$7)"
    )
    .bind(&sub.email)
    .bind(&sub.surname)
    .bind(&sub.lastname)
    .bind(&sub.address)
    .bind(&sub.city)
    .bind(&sub.postal_code)
    .bind(&sub.phone_number)
    .execute(&pool)
    .await;

    "ok"
}

// Update subscriber
async fn update_subscribe(
    State(pool): State<PgPool>,
    Json(sub): Json<Subscriber>,
) -> &'static str {
    let _ = sqlx::query(
        "UPDATE subscription \
         SET email=$1, surname=$2, lastname=$3, address=$4, city=$5, postal_code=$6, phone_number=$7 \
         WHERE id=$8"
    )
    .bind(&sub.email)
    .bind(&sub.surname)
    .bind(&sub.lastname)
    .bind(&sub.address)
    .bind(&sub.city)
    .bind(&sub.postal_code)
    .bind(&sub.phone_number)
    .bind(sub.id.unwrap_or(0))
    .execute(&pool)
    .await;

    "ok"
}

// Delete subscriber (RESTful via query param)
async fn delete_subscribe(
    State(pool): State<PgPool>,
    Query(params): Query<DeleteParams>, // ✅ Query, géén JSON body
) -> &'static str {
    // Voer de DELETE query uit op de database
    let result = sqlx::query("DELETE FROM subscription WHERE id=$1")
        .bind(params.id)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => "ok",               // Succes
        Err(e) => {
            eprintln!("Error deleting subscriber: {:?}", e);
            "error"
        }
    }
}


// Show all subscribers
async fn show_all_subscribers(
    State(pool): State<PgPool>,
) -> Json<Vec<Subscriber>> {
    let rows = sqlx::query_as!(
        Subscriber,
        "SELECT id, email, surname, lastname, address, city, postal_code, phone_number FROM subscription"
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    Json(rows)
}

#[tokio::main]
async fn main() {
    // Postgres connectie
    let db_url = "postgres://user:password@localhost:5432/subscriptions";
    let pool = PgPool::connect(db_url).await.unwrap();

    // CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Router
    let app = Router::new()
        .route("/subscribe", post(subscribe))
        .route("/update", put(update_subscribe))
        .route("/delete", delete(delete_subscribe)) // <-- nu echte DELETE
        .route("/all", get(show_all_subscribers))
        .with_state(pool)
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("✅ Backend running on http://{}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
