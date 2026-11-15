# 06 · Livrer — commits, NOTES, pull request

**Entrée** : décision « livrer » de l'humain à P4, `make check` vert.
**Sortie** : un historique de commits qui raconte le déroulé, `NOTES.md`, le texte de la pull request.

L'agent commite ; il ne pousse pas et n'ouvre pas la pull request : il en prépare le texte, l'humain exécute.

## Consignes

1. **Revérifie** : `make check` vert, et `make test`, la commande de référence du dépôt.
2. **Historique** : relis `git log --oneline <base>~1..HEAD` et `git status`. On attend un commit par étape :
   ```
   docs: spec <id>
   docs: plan <id>
   test: <n> tests rouges pour <id>
   feat: <résumé> (<id>)
   docs: revue <id>
   fix: <écart> (<id>)        (0 à 2, un par tour de B3)
   docs: notes <id>
   ```
   Conventional Commits, sans scope, une seule ligne, minuscules, impératif, pas de point final. S'il manque un commit ou si des fichiers restent non commités, commite-les, un chemin par `git add`.
3. **NOTES.md** : lance d'abord `make bilan`, qui ajoute au journal les totaux du cycle (itérations B2, tours B3, escalades, validations humaines). Puis écris NOTES.md, une page au plus, en premier jet à partir de `journal.md` et `revue.md` :
   - comment le cycle s'est déroulé : étapes, itérations de B2, tours de B3, escalades ;
   - ce qui a été confié à l'agent ;
   - ce que l'humain a vérifié ou corrigé à la main, aux portes P1 et P4 surtout ;
   - les écarts non bloquants de la revue, laissés en l'état ;
   - ce qu'on ferait avec plus de temps.
   Des faits seulement, chiffrés à partir du journal. L'humain le réécrit à la première personne.
4. **Pull request** : propose à l'humain un titre et une description :
   - titre : la ligne du commit `feat` ;
   - description : What (une phrase), Why (le ticket), Changes (un point par fichier), Testing (`make check`, `make test`, nombre de tests).

**Commit** : L'agent fait lui-même `git add` (un chemin par fichier) et `git commit` ; `docs: notes <id>` avec `NOTES.md`, `journal.md`. Une fois ce commit fait, supprime `.sdlc-step`.
