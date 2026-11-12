//! API de réservation de stands.

pub mod db;

use std::sync::{Arc, Mutex};

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use rusqlite::Connection;
use serde::Deserialize;
use serde_json::json;

use db::BookingError;

pub type Db = Arc<Mutex<Connection>>;

/// Construit l'application sur une connexion déjà ouverte (un fichier, ou la mémoire pour les tests).
pub fn app(conn: Connection) -> Router {
    db::init(&conn).expect("initialisation de la base");
    let state: Db = Arc::new(Mutex::new(conn));
    Router::new()
        .route("/api/show", get(get_show))
        .route("/api/stands", get(get_stands))
        .route("/api/bookings", get(get_bookings).post(post_booking))
        .with_state(state)
}

#[derive(Deserialize)]
struct NewBooking {
    stand_id: i64,
    exhibitor: String,
}

fn error(status: StatusCode, message: &str) -> Response {
    (status, Json(json!({ "error": message }))).into_response()
}

fn internal(e: rusqlite::Error) -> Response {
    eprintln!("erreur base : {e}");
    error(StatusCode::INTERNAL_SERVER_ERROR, "Erreur interne")
}

async fn get_show(State(db): State<Db>) -> Response {
    match db::show(&db.lock().unwrap()) {
        Ok(show) => Json(show).into_response(),
        Err(e) => internal(e),
    }
}

async fn get_stands(State(db): State<Db>) -> Response {
    match db::stands(&db.lock().unwrap()) {
        Ok(stands) => Json(stands).into_response(),
        Err(e) => internal(e),
    }
}

async fn get_bookings(State(db): State<Db>) -> Response {
    match db::bookings(&db.lock().unwrap()) {
        Ok(bookings) => Json(bookings).into_response(),
        Err(e) => internal(e),
    }
}

async fn post_booking(State(db): State<Db>, Json(body): Json<NewBooking>) -> Response {
    let exhibitor = body.exhibitor.trim();
    if exhibitor.is_empty() {
        return error(
            StatusCode::UNPROCESSABLE_ENTITY,
            "Le nom de l'exposant est obligatoire",
        );
    }
    match db::book(&db.lock().unwrap(), body.stand_id, exhibitor) {
        Ok(Ok(booking)) => (StatusCode::CREATED, Json(booking)).into_response(),
        Ok(Err(BookingError::UnknownStand)) => error(StatusCode::NOT_FOUND, "Stand inconnu"),
        Ok(Err(BookingError::AlreadyBooked)) => {
            error(StatusCode::CONFLICT, "Ce stand est déjà réservé")
        }
        Err(e) => internal(e),
    }
}
