# Journal <id>

Trace horodatée du cycle, la plus récente en bas. Presque tout s'y écrit sans l'agent :

| Entrée | Qui l'écrit |
| --- | --- |
| `03 Tests · P2` : résultat de `check-red`, une ligne par test | `make check-red` |
| `04 Implémenter · itération k/3` : VERT ou ROUGE, étape et erreurs | `make check` |
| `Diagnostic :` sous une itération rouge | l'agent |
| `05 Revue · tour t/2` : verdict du reviewer | l'agent |
| `P1`, `P4`, `escalade` : « Validation humaine (nom git) : … » | l'humain, `make valider P=… MSG="…"` |
| `06 Livrer · bilan` : totaux du cycle | `make bilan` |
