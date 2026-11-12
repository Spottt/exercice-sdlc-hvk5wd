use rusqlite::Connection;

#[tokio::main]
async fn main() {
    let path = std::env::var("DATABASE_PATH").unwrap_or_else(|_| "stands.db".into());
    let conn = Connection::open(&path).expect("ouverture de la base");
    // PORT=3001 cargo run si le port 3000 est déjà pris (penser à API_URL côté front).
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".into());
    let address = format!("127.0.0.1:{port}");
    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .expect("port indisponible");
    println!("API sur http://{address} (base : {path})");
    axum::serve(listener, stands_api::app(conn))
        .await
        .expect("serveur");
}
