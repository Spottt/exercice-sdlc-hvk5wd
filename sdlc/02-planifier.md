# 02 · Planifier — spec → tâches

**Entrée** : `docs/tickets/<id>/spec.md` validée à P1.
**Sortie** : `docs/tickets/<id>/tasks.md`, d'après [`templates/tasks.md`](templates/tasks.md).

La spec dit le quoi ; le plan dit le comment. Il donne à l'agent des unités petites et testables, et à l'humain un diff lisible par tâche.

## Consignes

1. Relis la spec et le code qu'elle touche. N'écris que `tasks.md`.
2. Découpe en **3 à 6 tâches**, chacune :
   - petite : un changement cohérent, relisible en quelques minutes ;
   - testable : reliée aux numéros de critères qu'elle fait passer (`C1, C3`), ou marquée « socle » si elle prépare sans faire passer de critère ;
   - bornée : liste les fichiers qu'elle touche, tous présents dans le bloc `files` de la spec.
3. **Ordonne** : le socle d'abord (types, erreurs, injection de dépendances comme la date du jour), puis les règles métier, puis la route, puis le front éventuel. Une tâche ne dépend que de tâches précédentes.
4. Chaque critère de la spec apparaît dans au moins une tâche. Un critère orphelin signale un trou : complète le plan, ou signale-le à l'humain.
5. Une tâche qui exige un fichier hors du bloc `files` : arrête-toi et dis-le. La spec se corrige à P1, pas en douce.

## Pas de porte

Le plan n'a pas de porte : un coup d'œil de l'humain suffit (l'ordre est-il logique ? la date est-elle injectée avant la règle qui l'utilise ?). Note le plan dans `journal.md` :

```
## <date heure> · 02 Planifier
<n> tâches ; critères couverts : C1…Cn ; remarque de l'humain s'il y en a une
```

**Commit** : L'agent fait lui-même `git add` (un chemin par fichier) et `git commit` ; `docs: plan <id>` avec `tasks.md`, `journal.md`.
