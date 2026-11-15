use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use http_body_util::BodyExt;
use rusqlite::Connection;
use serde_json::{json, Value};
use tower::ServiceExt;

const DELAI: &str =
    "Le changement de stand n'est plus possible à moins de 7 jours de l'ouverture du salon";

fn app_at(today: &str) -> Router {
    stands_api::app_at(Connection::open_in_memory().unwrap(), today)
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

async fn book(app: &Router, stand_id: i64, exhibitor: &str) -> i64 {
    let (status, booking) = call(
        app,
        "POST",
        "/api/bookings",
        Some(json!({ "stand_id": stand_id, "exhibitor": exhibitor })),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    booking["id"].as_i64().unwrap()
}

async fn change(
    app: &Router,
    booking_id: i64,
    stand_id: i64,
    exhibitor: &str,
) -> (StatusCode, Value) {
    call(
        app,
        "PATCH",
        &format!("/api/bookings/{booking_id}"),
        Some(json!({ "stand_id": stand_id, "exhibitor": exhibitor })),
    )
    .await
}

async fn bookings(app: &Router) -> Value {
    call(app, "GET", "/api/bookings", None).await.1
}

#[tokio::test]
async fn c1_changer_pour_un_stand_plus_cher_libere_l_ancien_et_monte_l_acompte() {
    let app = app_at("2027-01-10");
    let id = book(&app, 1, "Pépinières Durand").await;

    let (status, booking) = change(&app, id, 4, "Pépinières Durand").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(booking["stand_id"], 4);
    assert_eq!(booking["deposit_cents"], 72_000);
    let (_, stands) = call(&app, "GET", "/api/stands", None).await;
    let by_label = |label: &str| {
        stands
            .as_array()
            .unwrap()
            .iter()
            .find(|s| s["label"] == label)
            .unwrap()["booked_by"]
            .clone()
    };
    assert_eq!(by_label("A1"), Value::Null);
    assert_eq!(by_label("B1"), "Pépinières Durand");
}

#[tokio::test]
async fn c2_changer_pour_un_stand_moins_cher_baisse_l_acompte_arrondi_au_centime_inferieur() {
    let app = app_at("2027-01-10");
    let id = book(&app, 5, "Atelier Bois").await;

    let (status, booking) = change(&app, id, 3, "Atelier Bois").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(booking["deposit_cents"], 49_499);
}

#[tokio::test]
async fn c3_le_changement_est_accepte_a_j_moins_7() {
    let app = app_at("2027-01-15");
    let id = book(&app, 1, "Pépinières Durand").await;

    let (status, booking) = change(&app, id, 2, "Pépinières Durand").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(booking["stand_id"], 2);
}

#[tokio::test]
async fn c4_le_changement_est_refuse_a_j_moins_6_et_la_reservation_reste_inchangee() {
    let app = app_at("2027-01-16");
    let id = book(&app, 1, "Pépinières Durand").await;
    let before = bookings(&app).await;

    let (status, body) = change(&app, id, 2, "Pépinières Durand").await;

    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(body["error"], DELAI);
    assert_eq!(bookings(&app).await, before);
}

#[tokio::test]
async fn c5_un_stand_deja_reserve_par_un_autre_est_refuse_et_rien_ne_change() {
    let app = app_at("2027-01-10");
    let id = book(&app, 1, "Pépinières Durand").await;
    book(&app, 4, "Serres du Sud").await;
    let before = bookings(&app).await;

    let (status, body) = change(&app, id, 4, "Pépinières Durand").await;

    assert_eq!(status, StatusCode::CONFLICT);
    assert_eq!(body["error"], "Ce stand est déjà réservé");
    assert_eq!(bookings(&app).await, before);
}

#[tokio::test]
async fn c6_un_autre_nom_que_celui_de_la_reservation_est_refuse() {
    let app = app_at("2027-01-10");
    let id = book(&app, 1, "Pépinières Durand").await;
    let before = bookings(&app).await;

    let (status, body) = change(&app, id, 2, "Atelier Bois").await;

    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(
        body["error"],
        "Cette réservation est au nom d'un autre exposant"
    );
    assert_eq!(bookings(&app).await, before);
}

#[tokio::test]
async fn c7_une_reservation_inconnue_renvoie_404() {
    let app = app_at("2027-01-10");

    let (status, body) = change(&app, 999, 2, "Pépinières Durand").await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body["error"], "Réservation inconnue");
}

#[tokio::test]
async fn c8_un_stand_inconnu_renvoie_404() {
    let app = app_at("2027-01-10");
    let id = book(&app, 1, "Pépinières Durand").await;

    let (status, body) = change(&app, id, 999, "Pépinières Durand").await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body["error"], "Stand inconnu");
}
