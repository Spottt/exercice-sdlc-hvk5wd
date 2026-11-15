# 06 · Livrer — commits, NOTES, pull request

**Entrée** : décision « livrer » de l'humain à P4, `make check` vert.
**Sortie** : un historique de commits qui raconte le déroulé, `NOTES.md`, le texte de la pull request.

L'agent ne commite pas, ne pousse pas, n'ouvre pas la pull request : il prépare, l'humain exécute.

## Consignes

1. **Revérifie** : `make check` vert, et `make test` (ce que le jury lance).
2. **Historique** : relis `git log --oneline <base>~1..HEAD` et `git status`. Attendu, un commit par étape :
   ```
   docs: spec <id>
   docs: plan <id>
   test: <n> tests rouges pour <id>
   feat: <résumé> (<id>)
   docs: revue <id>
   fix: <écart> (<id>)        (0 à 2, un par tour de B3)
   docs: notes <id>
   ```
   Conventional Commits, sans scope, une seule ligne, minuscules, impératif, pas de point final. S'il manque un commit ou si des fichiers restent non commités, propose le bloc `git add` / `git commit` pour l'humain, un chemin par `git add`.
3. **NOTES.md**, une page au plus, premier jet à partir de `journal.md` et `revue.md` :
   - comment le cycle s'est déroulé : étapes, itérations de B2, tours de B3, escalades ;
   - ce qui a été confié à l'agent ;
   - ce que l'humain a vérifié ou corrigé à la main, aux portes P1 et P4 surtout ;
   - les écarts non bloquants de la revue, laissés en l'état ;
   - ce qu'on ferait avec plus de temps.
   Faits seulement, chiffrés depuis le journal. L'humain le réécrit à la première personne.
4. **Pull request** : propose titre et description pour l'humain :
   - titre : la ligne du commit `feat` ;
   - description : What (une phrase), Why (le ticket), Changes (un point par fichier), Testing (`make check`, `make test`, nombre de tests).

```
## <date heure> · 06 Livrer
make check : VERT · make test : VERT · commits : <n> · B2 : <k> itérations · B3 : <t> tours · escalades : <e>
```

**Commit proposé** : `docs: notes <id>` avec `NOTES.md`, `journal.md`. Puis `rm .sdlc-step`.
