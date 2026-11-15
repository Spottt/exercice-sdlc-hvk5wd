# Revue T-142, tour 1

**Verdict** : À CORRIGER
**Base** : `e8f12b99272762889a20b3c14e09434c5471bbfe` · **Fichiers relus** : `api/src/db.rs`, `api/src/lib.rs`, `api/tests/changement_stand.rs`, `docs/tickets/T-142/spec.md`, `sdlc/constitution.md`

## Critères
| Critère | Test | Couvert et correct ? |
| --- | --- | --- |
| C1 | `c1_changer_pour_un_stand_plus_cher_libere_l_ancien_et_monte_l_acompte` | oui : vérifie 200, `stand_id` 4, `deposit_cents` 72 000, A1 libre et B1 au nom de « Pépinières Durand » dans `GET /api/stands` |
| C2 | `c2_changer_pour_un_stand_moins_cher_baisse_l_acompte_arrondi_au_centime_inferieur` | oui : B2 vers A3, 200, 49 499 |
| C3 | `c3_le_changement_est_accepte_a_j_moins_7` | oui : date 2027-01-15, 200 |
| C4 | `c4_le_changement_est_refuse_a_j_moins_6_et_la_reservation_reste_inchangee` | oui : 422, message exact, `GET /api/bookings` identique avant et après |
| C5 | `c5_un_stand_deja_reserve_par_un_autre_est_refuse_et_rien_ne_change` | oui : 409, message exact, les deux réservations inchangées |
| C6 | `c6_un_autre_nom_que_celui_de_la_reservation_est_refuse` | oui : 403, message exact, réservation inchangée |
| C7 | `c7_une_reservation_inconnue_renvoie_404` | oui |
| C8 | `c8_un_stand_inconnu_renvoie_404` | oui |

## Écarts bloquants
1. `api/src/lib.rs:114` : `db.lock().unwrap()` est un nouvel `unwrap()` hors tests, ce que R8 interdit (« pas de `unwrap()` ni `expect()` nouveau hors tests »). Les autres handlers font pareil, mais ce code existait déjà ; ce nouveau handler ne peut pas s'en servir d'excuse. Correction attendue : gérer le verrou empoisonné sans paniquer, par exemple `let Ok(conn) = db.lock() else { return error(StatusCode::INTERNAL_SERVER_ERROR, "Erreur interne"); };` puis `db::change_stand(&conn, ...)`.

## Écarts non bloquants
1. `api/src/db.rs:202` : si la date injectée est invalide, `julianday` renvoie NULL et la lecture en `i64` échoue avec une 500. Seuls les tests injectent une date, donc l'impact est nul en production. Il suffirait de le signaler dans la doc de `app_at`.
2. `api/tests/changement_stand.rs` : aucun test pour une date postérieure à l'ouverture (jours négatifs, hypothèse 3), ni pour un nom avec espaces autour ou vide (hypothèse 2), ni pour le même stand (hypothèse 6). Le code traite bien ces trois cas (`days < 7`, `trim` puis comparaison exacte, un UPDATE sur le même stand ne viole pas l'unicité). Ces cas sont hors critères, à noter dans NOTES.md.
3. `api/src/lib.rs:131` : le « 7 » du message est écrit en dur alors que `CHANGE_DEADLINE_DAYS` existe. Ce n'est qu'une question de cohérence.
4. Un `id` de chemin non numérique ou un corps JSON invalide renvoie le rejet par défaut d'axum (400 ou 422 en texte brut), pas une erreur JSON en français. `POST /api/bookings` se comporte déjà ainsi : c'est hors ticket.

Vérifié et conforme : ordre des contrôles 404, 403, 422, 404, 409 (hypothèse 7). Borne `jours ≥ 7` incluse ; les dates sans heure donnent des écarts entiers, donc `CAST` ne tronque rien (hypothèse 3). `date('now','localtime')` en production (hypothèse 4). Acompte `prix × 30 / 100` en division entière (hypothèse 5). Le verrou est tenu pendant tout `change_stand`, et la seule écriture est un unique UPDATE : un refus ne change rien en base, et l'unicité de `stand_id` garantit le 409 même en cas de course. Rien ne sort du périmètre : pas de front, pas d'annulation. Spec et code concordent.

## Constitution
R1 scope : OK (`db.rs`, `lib.rs` et le test font partie du bloc `files` ; `app_at` est le socle prévu). · R2 tests gelés : OK (`api/tests/` sans diff depuis `b729f40`, et le contrôle « tests gelés » de `make check` passe). · R4 dépendances : OK (aucune). · R5 déterminisme : OK (`app_at` avec une date fixe, base mémoire par test, pas d'horloge). · R6 un test par critère : OK. · R8 erreurs et `unwrap` : messages en français conformes, mais voir l'écart bloquant 1. · R9 : `make check` est vert.

## Tour 2

**Verdict** : LIVRABLE
**Base** : `e8f12b99272762889a20b3c14e09434c5471bbfe` · **Fichiers relus** : `api/src/db.rs`, `api/src/lib.rs`, `api/tests/changement_stand.rs`, `docs/tickets/T-142/spec.md`, `docs/tickets/T-142/revue.md` (tour 1), `sdlc/constitution.md`

### Critères
| Critère | Test | Couvert et correct ? |
| --- | --- | --- |
| C1 | `c1_changer_pour_un_stand_plus_cher_libere_l_ancien_et_monte_l_acompte` | oui : vérifie 200, `stand_id` 4, `deposit_cents` 72 000, et dans `GET /api/stands` A1 libre et B1 au nom de « Pépinières Durand » |
| C2 | `c2_changer_pour_un_stand_moins_cher_baisse_l_acompte_arrondi_au_centime_inferieur` | oui : B2 vers A3, 200, 49 499 |
| C3 | `c3_le_changement_est_accepte_a_j_moins_7` | oui : date 2027-01-15, 200 |
| C4 | `c4_le_changement_est_refuse_a_j_moins_6_et_la_reservation_reste_inchangee` | oui : 422, message exact, `GET /api/bookings` identique avant et après |
| C5 | `c5_un_stand_deja_reserve_par_un_autre_est_refuse_et_rien_ne_change` | oui : 409, message exact, les deux réservations inchangées |
| C6 | `c6_un_autre_nom_que_celui_de_la_reservation_est_refuse` | oui : 403, message exact, réservation inchangée |
| C7 | `c7_une_reservation_inconnue_renvoie_404` | oui |
| C8 | `c8_un_stand_inconnu_renvoie_404` | oui |

### Écarts bloquants
Aucun. L'écart bloquant du tour 1 (le `db.lock().unwrap()` du nouveau handler) est corrigé dans `beacaae`. `api/src/lib.rs:114-116` contient maintenant `let Ok(conn) = db.lock() else { return error(StatusCode::INTERNAL_SERVER_ERROR, "Erreur interne"); };`. Le message est celui de `internal()`, et ce commit ne modifie que `lib.rs` et le journal.

### Écarts non bloquants
1. `api/src/db.rs:202` : si la date injectée est invalide, `julianday` renvoie NULL et la lecture en `i64` échoue avec une 500. Seul `app_at`, donc les tests, injecte une date : aucun impact en production. Reporté du tour 1.
2. `api/tests/changement_stand.rs` : aucun test pour une date après l'ouverture (hypothèse 3), un nom vide ou entouré d'espaces (hypothèse 2), ni une demande du stand déjà occupé (hypothèse 6). Le code traite bien ces cas. `post_booking` enregistre des noms passés par `trim` et jamais vides, donc un nom vide envoyé au PATCH reçoit bien un 403. Ces cas sont hors critères, à noter dans NOTES.md.
3. `api/src/lib.rs:134` : le « 7 » est écrit en dur dans le message alors que `CHANGE_DEADLINE_DAYS` existe. Simple question de cohérence.
4. `api/src/lib.rs:114` : quand le verrou est empoisonné, la 500 part sans `eprintln!`, contrairement à `internal()`. Ce n'est qu'une question de diagnostic.
5. Un `id` de chemin non numérique ou un corps JSON invalide reçoit le rejet par défaut d'axum, en texte brut. `POST /api/bookings` fait déjà la même chose : hors ticket.

Vérifié et conforme :
- Ordre des contrôles : 404, 403, 422, 404, 409 (hypothèse 7).
- Délai : borne `jours ≥ 7` incluse ; les jours négatifs sont refusés (hypothèse 3).
- Date du jour : `date('now','localtime')` en production (hypothèse 4).
- Acompte : `prix × 30 / 100` en division entière (hypothèse 5).
- Écriture : un seul UPDATE, fait avec le verrou tenu, et l'unicité de `stand_id` produit le 409. Un refus ne change donc rien en base.
- Périmètre : pas de front. La spec et le code concordent.

### Constitution
- R1 scope : OK (`make check-scope` vert ; `app_at` est le socle prévu).
- R2 tests gelés : OK (`api/tests/` n'a pas changé depuis `b729f40` ; le contrôle « tests gelés » de `make check` passe).
- R4 dépendances : OK (aucune nouvelle).
- R5 déterminisme : OK (`app_at` avec une date fixe, base mémoire par test).
- R6 un test par critère : OK.
- R8 erreurs et `unwrap` : OK. Les messages sont en français. Le nouveau code n'ajoute aucun `unwrap()`. Le seul `expect` de `router` existait déjà dans `app` et a simplement été déplacé.
- R9 : `make check` est vert (8 tests Rust, 3 tests web).
