# Notes — T-142, changer de stand jusqu'à J-7

## Déroulé du cycle
Les six étapes de `sdlc/` ont tourné dans une seule session, avec un commit par étape (`git log --oneline e8f12b9~1..HEAD`).

- **01 Cadrer** : le ticket était prêt, il n'a pas fallu de `questions.md`. La spec pose 8 hypothèses et 8 critères, en classe de risque élevée (acompte et dates).
- **02 Planifier** : 3 tâches. D'abord le socle de la date injectée, puis la règle dans `db.rs`, puis la route `PATCH`.
- **03 Tests** : 8 tests rouges, un par critère, plus le socle `app_at` qui ne fait rien. `make check-red` est passé du premier coup.
- **04 Implémenter** : `make check` vert dès l'itération 1.
- **05 Revue** : 2 tours de B3. Le tour 1 rend « À CORRIGER » à cause d'un nouvel `unwrap()` sur le verrou (R8). Après le `fix`, le tour 2 rend « LIVRABLE ».
- Bilan du journal : 2 itérations B2 (une par passage en 04), 2 tours B3, 0 escalade, 2 validations humaines.

## Ce qui a été confié à l'agent
La rédaction de la spec et du plan, les tests, l'implémentation et la correction de revue, ainsi que les commits de chaque étape. La revue a été faite par un sous-agent au contexte neuf, qui ne voyait que la spec, la base et le diff.

## Ce que l'humain a vérifié
- **P1** : les 8 hypothèses ont été validées telles quelles. Les plus importantes : la borne J-7 incluse, la date du jour lue en heure locale du serveur et l'acompte arrondi à l'entier inférieur.
- **P4** : la revue et le diff ont été lus (risque élevé), avec pour décision « livrer ».

## Écarts non bloquants laissés en l'état
1. Une date injectée invalide fait renvoyer une 500 par `julianday`. Seuls les tests injectent une date.
2. Il n'y a pas de test pour une date après l'ouverture, un nom vide ou entouré d'espaces, ni une demande du stand déjà occupé. Le code traite bien ces cas.
3. Le « 7 » est écrit en dur dans le message d'erreur, alors que la constante `CHANGE_DEADLINE_DAYS` existe.
4. Quand le verrou est empoisonné, la 500 part sans `eprintln!`.
5. Un id non numérique ou un JSON invalide reçoit le rejet par défaut d'axum, en texte brut. `POST /api/bookings` fait déjà la même chose.

## Avec plus de temps
- Le front : un bouton « Changer de stand », reporté par l'hypothèse 8.
- Des tests pour les cas limites du point 2.
- Lire la date du jour dans le fuseau du salon plutôt que dans celui du serveur.
- Renvoyer des erreurs JSON en français pour les rejets d'extraction d'axum.
