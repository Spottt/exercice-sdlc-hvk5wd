# 03 · Tests — un test rouge par critère

**Entrée** : `spec.md`, `tasks.md`.
**Sortie** : le fichier de tests listé dans le bloc `files` de la spec (par exemple `api/tests/<sujet>.rs`), rouge.

Un test qui n'a jamais été rouge ne prouve rien. Cette étape écrit les tests **avant** le code, et la porte vérifie qu'ils échouent pour la bonne raison.

## Consignes

1. **Un test par critère**, nommé d'après lui : `c1_<résumé_du_critère>`. Le corps suit le schéma « étant donné / quand / alors » du critère, rien de plus.
2. Écris uniquement dans les fichiers de tests listés dans le bloc `files`. Ne touche ni à `api/src/` ni à `web/src/` hors `*.test.ts`. N'écris aucune implémentation.
3. **Teste à travers l'API HTTP**, dans le style de `api/tests/api.rs` (helpers `app()` et `call()`, `oneshot` sur le routeur). Ainsi les tests compilent sans le nouveau code. Si un test a besoin d'un point d'entrée qui n'existe pas encore (par exemple `app_at(conn, today)` pour injecter la date), il ne compilera pas : arrête-toi et demande à l'humain d'arbitrer, n'écris pas ce point d'entrée toi-même.
4. **Déterminisme** (constitution R5) :
   - date du jour injectée, jamais lue sur l'horloge : pas de `SystemTime::now`, `Local::now`, `Utc::now` ;
   - base SQLite en mémoire neuve par test ; aucun test ne dépend d'un autre ni de l'ordre ;
   - pas de `sleep`, pas de réseau, pas de valeur aléatoire.
5. Les tests déjà présents ne bougent pas.

## Porte P2 · Rouge — `make check-red`

Lance `make check-red`. La cible vérifie, dans l'ordre : aucune horloge ni `sleep` dans `api/tests/` ; seuls des tests (et `docs/tickets/<id>/`) ont bougé ; tout compile ; les anciens tests passent ; **chaque** nouveau test échoue.

La cible ajoute elle-même son résultat au journal, une ligne par test ; tu n'as rien à y copier.

- OK : passe au commit.
- Échec : corrige les tests et relance. Si un critère semble impossible à rendre rouge (le comportement existe déjà), arrête-toi : c'est une question pour l'humain, la spec est peut-être fausse.

**Commit** : L'agent fait lui-même `git add` (un chemin par fichier) et `git commit` ; `test: <n> tests rouges pour <id>`. Ce commit gèle les tests : à partir d'ici, `make check` refuse toute modification des tests.
