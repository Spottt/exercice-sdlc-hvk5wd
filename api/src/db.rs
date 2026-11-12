//! Accès à la base (SQLite, un seul fichier : rien à installer).

use rusqlite::{params, Connection, ErrorCode};
use serde::Serialize;

/// Acompte demandé à la réservation, en pourcentage du prix du stand.
pub const DEPOSIT_PERCENT: i64 = 30;

#[derive(Debug, Serialize)]
pub struct Show {
    pub name: String,
    /// Date d'ouverture du salon, au format AAAA-MM-JJ.
    pub opens_on: String,
}

#[derive(Debug, Serialize)]
pub struct Stand {
    pub id: i64,
    pub label: String,
    pub area_m2: i64,
    pub price_cents: i64,
    /// Nom de l'exposant qui a réservé le stand, s'il y en a un.
    pub booked_by: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Booking {
    pub id: i64,
    pub stand_id: i64,
    pub exhibitor: String,
    pub deposit_cents: i64,
}

#[derive(Debug, PartialEq)]
pub enum BookingError {
    UnknownStand,
    AlreadyBooked,
}

/// Crée les tables et le jeu de données de départ si la base est vide.
pub fn init(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS show (
             id INTEGER PRIMARY KEY CHECK (id = 1),
             name TEXT NOT NULL,
             opens_on TEXT NOT NULL
         );
         CREATE TABLE IF NOT EXISTS stands (
             id INTEGER PRIMARY KEY,
             label TEXT NOT NULL UNIQUE,
             area_m2 INTEGER NOT NULL,
             price_cents INTEGER NOT NULL
         );
         CREATE TABLE IF NOT EXISTS bookings (
             id INTEGER PRIMARY KEY,
             stand_id INTEGER NOT NULL UNIQUE REFERENCES stands(id),
             exhibitor TEXT NOT NULL,
             deposit_cents INTEGER NOT NULL
         );",
    )?;
    let count: i64 = conn.query_row("SELECT count(*) FROM stands", [], |r| r.get(0))?;
    if count == 0 {
        conn.execute(
            "INSERT INTO show (id, name, opens_on) VALUES (1, 'Salon Habitat & Jardin', '2027-01-22')",
            [],
        )?;
        for (label, area, price) in [
            ("A1", 9, 120_000),
            ("A2", 9, 120_000),
            ("A3", 12, 164_999),
            ("B1", 18, 240_000),
            ("B2", 18, 255_000),
            ("C1", 36, 450_000),
        ] {
            conn.execute(
                "INSERT INTO stands (label, area_m2, price_cents) VALUES (?1, ?2, ?3)",
                params![label, area, price],
            )?;
        }
    }
    Ok(())
}

pub fn show(conn: &Connection) -> rusqlite::Result<Show> {
    conn.query_row("SELECT name, opens_on FROM show WHERE id = 1", [], |r| {
        Ok(Show {
            name: r.get(0)?,
            opens_on: r.get(1)?,
        })
    })
}

pub fn stands(conn: &Connection) -> rusqlite::Result<Vec<Stand>> {
    let mut stmt = conn.prepare(
        "SELECT s.id, s.label, s.area_m2, s.price_cents, b.exhibitor
         FROM stands s LEFT JOIN bookings b ON b.stand_id = s.id
         ORDER BY s.label",
    )?;
    let rows = stmt.query_map([], |r| {
        Ok(Stand {
            id: r.get(0)?,
            label: r.get(1)?,
            area_m2: r.get(2)?,
            price_cents: r.get(3)?,
            booked_by: r.get(4)?,
        })
    })?;
    rows.collect()
}

pub fn bookings(conn: &Connection) -> rusqlite::Result<Vec<Booking>> {
    let mut stmt =
        conn.prepare("SELECT id, stand_id, exhibitor, deposit_cents FROM bookings ORDER BY id")?;
    let rows = stmt.query_map([], |r| {
        Ok(Booking {
            id: r.get(0)?,
            stand_id: r.get(1)?,
            exhibitor: r.get(2)?,
            deposit_cents: r.get(3)?,
        })
    })?;
    rows.collect()
}

/// Réserve un stand : l'acompte est calculé ici, et la contrainte d'unicité de la base
/// garantit qu'un stand n'est réservé qu'une fois.
pub fn book(
    conn: &Connection,
    stand_id: i64,
    exhibitor: &str,
) -> rusqlite::Result<Result<Booking, BookingError>> {
    let price: Option<i64> = conn
        .query_row(
            "SELECT price_cents FROM stands WHERE id = ?1",
            [stand_id],
            |r| r.get(0),
        )
        .map(Some)
        .or_else(|e| {
            if e == rusqlite::Error::QueryReturnedNoRows {
                Ok(None)
            } else {
                Err(e)
            }
        })?;
    let Some(price) = price else {
        return Ok(Err(BookingError::UnknownStand));
    };
    let deposit_cents = price * DEPOSIT_PERCENT / 100;
    match conn.execute(
        "INSERT INTO bookings (stand_id, exhibitor, deposit_cents) VALUES (?1, ?2, ?3)",
        params![stand_id, exhibitor, deposit_cents],
    ) {
        Ok(_) => Ok(Ok(Booking {
            id: conn.last_insert_rowid(),
            stand_id,
            exhibitor: exhibitor.to_string(),
            deposit_cents,
        })),
        Err(rusqlite::Error::SqliteFailure(e, _)) if e.code == ErrorCode::ConstraintViolation => {
            Ok(Err(BookingError::AlreadyBooked))
        }
        Err(e) => Err(e),
    }
}
