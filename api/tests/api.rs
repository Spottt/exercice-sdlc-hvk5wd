use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use http_body_util::BodyExt;
use rusqlite::Connection;
use serde_json::{json, Value};
use tower::ServiceExt;

fn app() -> Router {
    stands_api::app(Connection::open_in_memory().unwrap())
}

async fn call(app: &Router, method: &str, path: &str, body: Option<Value>) -> (StatusCode, Value) {
    let request = Request::builder()
        .method(method)
        .uri(path)
        .header("content-type", "application/json")
        .body(
            body.map(|b| Body::from(b.to_string()))
                .unwrap_or_else(Body::empty),
        )
        .unwrap();
    let response = app.clone().oneshot(request).await.unwrap();
    let status = response.status();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}

#[tokio::test]
async fn le_salon_et_ses_stands_sont_listes() {
    let app = app();
    let (status, show) = call(&app, "GET", "/api/show", None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(show["opens_on"], "2027-01-22");

    let (_, stands) = call(&app, "GET", "/api/stands", None).await;
    assert_eq!(stands.as_array().unwrap().len(), 6);
    assert_eq!(stands[0]["label"], "A1");
    assert_eq!(stands[0]["booked_by"], Value::Null);
}

#[tokio::test]
async fn reserver_un_stand_calcule_un_acompte_de_30_pour_cent() {
    let app = app();
    let (status, booking) = call(
        &app,
        "POST",
        "/api/bookings",
        Some(json!({ "stand_id": 1, "exhibitor": "Pépinières Durand" })),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(booking["deposit_cents"], 36_000);

    let (_, stands) = call(&app, "GET", "/api/stands", None).await;
    assert_eq!(stands[0]["booked_by"], "Pépinières Durand");
}

#[tokio::test]
async fn un_stand_ne_peut_etre_reserve_qu_une_fois() {
    let app = app();
    call(
        &app,
        "POST",
        "/api/bookings",
        Some(json!({ "stand_id": 2, "exhibitor": "Atelier Bois" })),
    )
    .await;
    let (status, body) = call(
        &app,
        "POST",
        "/api/bookings",
        Some(json!({ "stand_id": 2, "exhibitor": "Serres du Sud" })),
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);
    assert_eq!(body["error"], "Ce stand est déjà réservé");
}

#[tokio::test]
async fn les_demandes_invalides_sont_refusees() {
    let app = app();
    let (status, _) = call(
        &app,
        "POST",
        "/api/bookings",
        Some(json!({ "stand_id": 999, "exhibitor": "X" })),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    let (status, _) = call(
        &app,
        "POST",
        "/api/bookings",
        Some(json!({ "stand_id": 1, "exhibitor": "  " })),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
}
