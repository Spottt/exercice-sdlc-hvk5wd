# 04 · Implémenter — jusqu'au vert, 3 itérations max

**Entrée** : `spec.md`, `tasks.md`, les tests commités à l'étape 03.
**Sortie** : le code, `make check` vert, chaque itération dans `journal.md`.

## Consignes

1. Les tests sont **gelés** (constitution R2) : le hook refuse toute écriture sous `api/tests/` et dans `*.test.ts`, et `make check` échoue si un test diffère du commit. Si un test te paraît faux, n'y touche pas : explique-le dans le journal et arrête-toi pour l'humain.
2. Ne modifie que des fichiers du bloc `files` de la spec.
3. Avance **tâche par tâche**, dans l'ordre de `tasks.md`. Implémente le minimum qui fait passer les critères de la tâche ; pas de fonctionnalité absente de la spec.
4. Suis le style du code existant : erreurs dans l'enum du module, messages en français comme ceux de `lib.rs`, pas de `unwrap()`/`expect()` hors tests, pas de dépendance nouvelle non prévue par la spec.
5. Les hypothèses tranchées à P1 font foi. Si le code révèle un cas que la spec ne règle pas, ne le devine pas : note-le et demande.

## Boucle B2 — 3 itérations max

Une itération = une modification du code puis `make check`.

```
## <date heure> · 04 Implémenter · itération <k>/3
Tâches : <numéros>  ·  make check : VERT | ROUGE
<si rouge : la cible en échec et les lignes d'erreur décisives>
<si rouge : hypothèse sur la cause, et ce qui change à l'itération suivante>
```

Avant chaque nouvelle itération, **relis l'historique des itérations précédentes** dans le journal. Ne rejoue pas une correction déjà tentée : si la même erreur revient, change d'hypothèse.

- **P3 vert** à l'itération k ≤ 3 : passe à 05.
- **Rouge à la 3e itération** : arrête-toi. Écris « Escalade : B2 atteint 3 itérations » avec le diagnostic, et rends la main. L'humain reprend le clavier et note dans le journal ce qui a été corrigé, et NOTES.md le reprendra.

## Porte P3 · Vert — `make check`

`cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `npm test`, `check-scope`, tests gelés. Vert, rien d'autre ne compte : ni « ça devrait marcher », ni un avis sur ton propre code.

**Commit proposé** : `feat: <résumé du ticket> (<id>)` avec les fichiers du code et `journal.md`.

Lors d'un retour de la boucle B3 (corrections de revue), même règles, même compteur remis à zéro, et le commit proposé devient `fix: <écart corrigé> (<id>)`.
