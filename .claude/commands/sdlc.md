---
description: SDLC complet — enchaîne les étapes et ne s'arrête qu'aux portes humaines et aux bornes
argument-hint: <id du ticket, ex. T-142>
---
Fais avancer le ticket `$ARGUMENTS` dans le cycle `sdlc/` jusqu'au prochain arrêt obligatoire. Lis d'abord `sdlc/README.md` et `sdlc/constitution.md`.

## 1. Situe le ticket

Lis `docs/tickets/$ARGUMENTS/journal.md` et `git log --oneline`, puis reprends à la première étape non faite :

| Constat | Reprendre à |
| --- | --- |
| Pas de `spec.md`, ou `questions.md` complété par les réponses de l'humain | 01 Cadrer |
| `spec.md` écrite, pas d'entrée `· P1 · validation humaine` | arrêt : P1 en attente |
| P1 signée, pas de commit `docs: spec $ARGUMENTS` | fin de 01 : reporter les décisions, commiter |
| Pas de commit `docs: plan`, `test:`, `feat:` ou `docs: revue` | l'étape correspondante, dans cet ordre |
| Revue commitée, pas d'entrée `· P4 · validation humaine` postérieure | arrêt : P4 en attente |
| Dernière P4 : « corriger » | 04 avec les corrections demandées, puis 05 (tour suivant) |
| Dernière P4 : « livrer » | 06 Livrer |

## 2. Enchaîne

Pour chaque étape : écris `<étape> $ARGUMENTS` dans `.sdlc-step` (`cadrer`, `planifier`, `tests`, `implementer`, `revue`, `livrer`) avant toute autre action, suis `sdlc/0X-*.md`, termine par son commit, puis passe à la suivante sans attendre.

## 3. Arrête-toi seulement ici

- **P1** après 01, ou `questions.md` écrit (boucle B1) ;
- **P4** après 05 ;
- une **borne** atteinte : 3 itérations rouges en B2, bloquants encore présents au tour 2 de B3 ;
- une étape qui dit de s'arrêter : test qui paraît faux, fichier hors du bloc `files`, critère impossible à rendre rouge, cas que la spec ne règle pas ;
- la **fin de 06**.

À chaque arrêt, dis en trois lignes au plus où en est le ticket et la commande exacte que l'humain doit lancer, par exemple `! make valider P=P1 MSG="…"`, puis `/sdlc $ARGUMENTS` pour reprendre.

Ne lance jamais `make valider` et n'écris jamais « Validation humaine » : c'est la signature de l'humain, et le hook le refuse.
