# Spec T-142 — Changer de stand jusqu'à J-7

## Problème
Un exposant qui a réservé un stand veut en changer lui-même, sans appeler l'organisateur, tant que le salon ouvre dans 7 jours ou plus. L'acompte (30 % du prix) doit suivre le prix du nouveau stand, et un stand ne doit jamais être réservé par deux exposants.

## Comportement attendu
`PATCH /api/bookings/{id}` avec le corps `{ "stand_id": <nouveau stand>, "exhibitor": "<nom saisi à la réservation>" }`.

- Succès : `200`, corps = la réservation à jour `{ "id", "stand_id", "exhibitor", "deposit_cents" }`, `deposit_cents` recalculé sur le prix du nouveau stand. L'ancien stand redevient libre dans `GET /api/stands`.
- Réservation inconnue : `404 {"error": "Réservation inconnue"}`.
- Nom différent de celui de la réservation : `403 {"error": "Cette réservation est au nom d'un autre exposant"}`.
- Moins de 7 jours avant l'ouverture : `422 {"error": "Le changement de stand n'est plus possible à moins de 7 jours de l'ouverture du salon"}`.
- Nouveau stand inconnu : `404 {"error": "Stand inconnu"}`.
- Nouveau stand déjà réservé par un autre : `409 {"error": "Ce stand est déjà réservé"}`.
- En cas de refus, rien ne change en base.

## Hypothèses (à trancher à P1)
1. **Route** — proposé : `PATCH /api/bookings/{id}`, on modifie le stand d'une réservation existante (même id) plutôt que d'annuler puis recréer. — **humain : validé à P1**
2. **Identification** — proposé : sans connexion, l'exposant prouve qu'il est le titulaire en renvoyant le nom saisi à la réservation ; comparaison exacte après `trim` (casse et accents comptent). Nom différent ou vide : `403`. — **humain : validé à P1**
3. **Borne J-7 incluse** — proposé : `jours = date d'ouverture − date du jour`, changement accepté si `jours ≥ 7`. Salon le 2027-01-22 : le 2027-01-15 (J-7) est accepté, le 2027-01-16 (J-6) est refusé ; une date après l'ouverture (jours négatifs) est refusée aussi. — **humain : validé à P1**
4. **Date du jour et fuseau** — proposé : en production, date du jour lue en heure locale du serveur (`date('now','localtime')` de SQLite), le serveur étant supposé à l'heure de Paris comme le salon. Le 2027-01-16 à 00h05 heure de Paris, on est J-6 : refusé (en UTC il serait encore le 15 à 23h05 et le changement passerait). Dans les tests, la date est injectée par `app_at(conn, "AAAA-MM-JJ")`. — **humain : validé à P1**
5. **Acompte** — proposé : `deposit_cents` est remplacé par `prix du nouveau stand × 30 / 100`, arrondi à l'entier inférieur comme à la réservation. Exemple : B2 (2 550,00 €, acompte 765,00 €) vers A3 (1 649,99 €) : nouvel acompte 49 499 centimes (494,99 €). Pas de trace du trop-perçu ni du complément : il n'y a pas de paiement. — **humain : validé à P1**
6. **Même stand** — proposé : demander le stand qu'on occupe déjà renvoie `200` avec la réservation inchangée, sans cas particulier. — **humain : validé à P1**
7. **Ordre des contrôles** — proposé : réservation inconnue (404), nom (403), délai (422), stand inconnu (404), stand pris (409). Le délai passe avant le stand : à J-6, tout changement est refusé quel que soit le stand demandé. — **humain : validé à P1**
8. **Front** — proposé : hors périmètre de ce ticket (le ticket le donne en option) ; l'API seule. — **humain : validé à P1**

## Critères d'acceptation
Salon du 2027-01-22, stands du jeu de départ (A1 id 1 à 120 000, A3 id 3 à 164 999, B1 id 4 à 240 000, B2 id 5 à 255 000), date du jour 2027-01-10 sauf mention contraire.

- **C1** — Étant donné une réservation de A1 par « Pépinières Durand », quand elle demande B1, alors `200`, `stand_id` 4, `deposit_cents` 72 000, et `GET /api/stands` montre A1 libre et B1 réservé par « Pépinières Durand ».
- **C2** — Étant donné une réservation de B2 (acompte 76 500), quand l'exposant demande A3, alors `200` et `deposit_cents` 49 499.
- **C3** — Étant donné la date du jour 2027-01-15 (J-7), quand un exposant change de stand, alors `200`.
- **C4** — Étant donné la date du jour 2027-01-16 (J-6), quand un exposant change de stand, alors `422` avec le message du délai, et `GET /api/bookings` montre la réservation inchangée.
- **C5** — Étant donné B1 réservé par « Serres du Sud », quand « Pépinières Durand » demande B1, alors `409 "Ce stand est déjà réservé"` et les deux réservations sont inchangées.
- **C6** — Étant donné une réservation de « Pépinières Durand », quand la demande porte le nom « Atelier Bois », alors `403` avec le message du titulaire et la réservation est inchangée.
- **C7** — Étant donné aucune réservation d'id 999, quand on la modifie, alors `404 "Réservation inconnue"`.
- **C8** — Étant donné une réservation existante, quand on demande le stand 999, alors `404 "Stand inconnu"`.

## Hors périmètre
- Front : option du ticket, reportée (hypothèse 8).
- Annulation de réservation, historique des changements, remboursement ou encaissement : absents du ticket.
- Connexion utilisateur : exclue par le ticket.

## Classe de risque
élevée — argent (acompte recalculé) et dates (délai J-7, fuseau).

## Fichiers
```files
api/src/db.rs
api/src/lib.rs
api/tests/changement_stand.rs
```

Socle, seulement si les tests ont besoin d'un point d'entrée qui n'existe pas (par exemple une date injectée) ; fichiers déjà présents dans `files` :
```socle
api/src/lib.rs
```
