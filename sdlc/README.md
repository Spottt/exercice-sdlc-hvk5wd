# SDLC — du ticket à la pull request

Un cycle minimal pour qu'un agent de code traite un ticket proprement : six étapes, quatre portes dont deux humaines, trois boucles bornées. Tout tourne dans une seule session Claude Code ; seule la revue part dans un sous-agent au contexte neuf. L'humain est la personne qui pilote l'agent et signe ses décisions dans le journal.

## Lancer le cycle

```
/sdlc-cadrer T-142      01  ticket -> spec.md (ou questions.md)        porte P1 · humain
/sdlc-planifier T-142   02  spec -> tasks.md
/sdlc-tests T-142       03  un test rouge par critère                  porte P2 · make check-red
/sdlc-implementer T-142 04  code jusqu'au vert, boucle B2              porte P3 · make check
/sdlc-revue T-142       05  sous-agent reviewer -> revue.md, boucle B3 porte P4 · humain
/sdlc-livrer T-142      06  commits, NOTES.md, pull request
```

Chaque commande écrit `<étape> <ticket>` dans `.sdlc-step`, puis suit le fichier `sdlc/0X-*.md` correspondant. Le hook `.claude/hooks/guard.sh` lit `.sdlc-step` pour savoir ce qui est permis. Les traces vont dans `docs/tickets/<id>/` : `spec.md`, `tasks.md`, `revue.md`, `journal.md` (modèles dans `templates/`).

## Les quatre portes

Une porte est un endroit où le cycle ne continue pas tout seul. Le jugement humain se met aux portes, pas sur chaque diff.

| Porte | Après | Qui | Vérifie |
| --- | --- | --- | --- |
| P1 · Spec | 01 | Humain | Hypothèses tranchées, hors périmètre écrit, classe de risque déclarée |
| P2 · Rouge | 03 | `make check-red` | Nouveaux tests compilent et échouent, anciens verts, seuls des tests ont bougé, aucune horloge |
| P3 · Vert | 04 | `make check` | fmt, clippy `-D warnings`, `cargo test`, `npm test`, scope, tests gelés |
| P4 · Merge | 05 | Humain | Lit `revue.md` (et le diff si risque élevé) ; décide : livrer, corriger, reprendre à la main |

Chaque passage de porte est écrit dans `journal.md` : sortie de la commande, ou « Validation humaine (<nom>) : … » avec le motif.

## Les trois boucles, toutes bornées

| Boucle | Déclencheur | Retour vers | Borne | Au-delà |
| --- | --- | --- | --- | --- |
| B1 · Clarification | 01 produit `questions.md` | 01, avec les réponses de l'humain | 1 tour | Ticket pas prêt : on s'arrête et on le dit |
| B2 · Implémentation | P3 rouge | 04, avec la sortie de `make check` et l'historique | 3 itérations | Escalade : l'humain reprend le clavier, journal et NOTES le disent |
| B3 · Revue | écarts bloquants dans `revue.md` | 04 puis 05 | 2 tours | L'humain tranche à P4 |

Sans borne, un agent rejoue la même correction ; avec l'historique des échecs et une limite, il converge ou rend la main.

## Qui décide quoi

- **L'agent** rédige, teste, implémente, et propose les commandes de commit.
- **Les contrôles mécaniques** (`make`, hook) disent pass/fail ; un avis d'agent sur son propre code ne compte pas.
- **L'humain** valide la spec (P1), décide de la livraison (P4), reprend la main au-delà d'une borne, lance les commits.
- **Les règles** sont dans [`constitution.md`](constitution.md) ; chacune nomme le contrôle qui l'applique.
