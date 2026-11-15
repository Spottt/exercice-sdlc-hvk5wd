# Plan T-142

| # | Tâche | Critères | Fichiers |
| --- | --- | --- | --- |
| 1 | Socle : `app_at(conn, today)` injecte la date du jour dans l'état du routeur ; `app(conn)` garde la date du serveur (`date('now','localtime')`) ; aucune route nouvelle | socle | `api/src/lib.rs` |
| 2 | Règle métier `db::change_stand(conn, booking_id, stand_id, exhibitor, today)` : réservation inconnue, titulaire, délai J-7 calculé par SQLite (`julianday`), stand inconnu, acompte recalculé, `UPDATE` unique dont la contrainte d'unicité donne le conflit ; erreurs dans un enum `ChangeError` | C2, C3, C4, C5, C6, C7, C8 | `api/src/db.rs` |
| 3 | Route `PATCH /api/bookings/{id}` et traduction de `ChangeError` en codes HTTP et messages français de la spec | C1, C3, C4, C5, C6, C7, C8 | `api/src/lib.rs` |

Couverture : C1 → 3 · C2 → 2 · C3 → 2, 3 · C4 → 2, 3 · C5 → 2, 3 · C6 → 2, 3 · C7 → 2, 3 · C8 → 2, 3

Ordre : la date est injectée (1) avant la règle qui l'utilise (2), la route vient en dernier.
