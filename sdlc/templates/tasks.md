# Plan <id>

| # | Tâche | Critères | Fichiers |
| --- | --- | --- | --- |
| 1 | <socle : types, erreurs, injection de dépendances> | socle | `api/src/…` |
| 2 | <règle métier> | C2, C3 | `api/src/…` |
| 3 | <route et traduction des erreurs en codes HTTP> | C1, C4 | `api/src/…` |

Couverture : C1 → 3 · C2 → 2 · C3 → 2 · C4 → 3 · … (chaque critère au moins une fois)

Ordre : <une phrase si l'ordre n'est pas évident>
