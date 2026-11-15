# Spec <id> — <titre court>

## Problème
<2 ou 3 phrases, dans les termes de l'utilisateur>

## Comportement attendu
<ce qu'on observe via l'API : route, code HTTP, corps de réponse ; idem pour le front si concerné>

## Hypothèses (à trancher à P1)
1. <point laissé ouvert par le ticket> — proposé : <décision> — **<nom> : validé | corrigé en …**
2. ...

## Critères d'acceptation
- **C1** — Étant donné <contexte>, quand <action>, alors <résultat observable>.
- **C2** — ...

## Hors périmètre
- <ce qu'on ne fait pas, et pourquoi en une demi-ligne>

## Classe de risque
<élevée | normale> — <raison : argent, dates, schéma, auth, contrôles… ou « aucun de ces cas »>

## Fichiers
```files
api/src/<fichier>.rs
api/tests/<sujet>.rs
```
<dépendance nouvelle : une ligne de justification, sinon rien>

Socle, seulement si les tests ont besoin d'un point d'entrée qui n'existe pas (par exemple une date injectée) ; fichiers déjà présents dans `files` :
```socle
api/src/<fichier>.rs
```
