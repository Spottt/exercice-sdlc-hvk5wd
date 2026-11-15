# 05 · Revue — contexte neuf, 2 tours max

**Entrée** : `spec.md`, le diff depuis le commit de la spec, `make check` vert.
**Sortie** : `docs/tickets/<id>/revue.md`, d'après [`templates/revue.md`](templates/revue.md), et la décision de l'humain.

L'agent qui a écrit le code ne peut pas le juger : son avis valide surtout ses propres hypothèses. La revue part donc dans le sous-agent `reviewer` (`.claude/agents/reviewer.md`), qui ne voit que la spec, le diff et les tests, et ne peut rien écrire.

## Consignes

1. Calcule la base : `git log --diff-filter=A --format=%H -- docs/tickets/<id>/spec.md | tail -1`.
2. Lance le sous-agent `reviewer` avec seulement : l'id du ticket, le chemin de la spec, la base, et le numéro du tour (1 ou 2). Ne lui donne ni ton raisonnement, ni ton avis sur le code, ni le journal.
3. Écris sa réponse telle quelle dans `revue.md` (au tour 2, ajoute-la sous une section « Tour 2 »).

## Boucle B3 — 2 tours max

- **Verdict LIVRABLE** : passe à la porte P4.
- **Écarts bloquants** (un critère non couvert ou faux, une règle de la constitution violée, un bug de correction) : retour à 04 avec la liste des écarts bloquants, `/sdlc-implementer <id>` puis `make check`, puis nouvelle revue, tour 2.
- Les écarts **non bloquants** ne relancent pas de boucle : ils vont dans NOTES.md.
- **Encore des bloquants au tour 2** : arrête-toi, l'humain tranche à P4.

## Porte P4 · Merge — humain

Présente à l'humain : le verdict, les écarts, la classe de risque. L'humain lit `revue.md` ; en classe **élevée**, il faut aussi lire le diff (`git diff <base>`). Décision :

- **livrer** : on passe à 06 ;
- **corriger** : l'humain dit quoi corriger, retour à 04 (si la borne B3 le permet) ;
- **reprendre à la main** : l'humain corrige à la main, et le note.

```
## <date heure> · 05 Revue · tour <t>/2 · P4
Verdict reviewer : <LIVRABLE | À CORRIGER>, <n> bloquants, <m> non bloquants
Validation humaine (<nom>) : <livrer | corriger | reprendre>, motif : <…>
```

**Commit proposé** : `docs: revue <id>` avec `revue.md`, `journal.md` (plus un `fix: … (<id>)` par tour de correction).
