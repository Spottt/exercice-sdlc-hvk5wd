# Exercice technique Spottt

Bonjour, et merci de prendre ce temps.

Chez Spottt, une grande partie du code est écrite par des agents IA. Le travail du lead developer est de construire
le cadre qui rend ce code fiable : c'est ce que nous vous demandons de faire ici, en petit.

## Votre mission

**Construisez un SDLC minimal (un cycle de développement) en fichiers Markdown, qui permet à un agent IA de traiter
un ticket proprement. Puis faites-le tourner sur le ticket [`TICKET.md`](TICKET.md).**

Comptez **3 heures maximum**. Nous préférons un petit SDLC qui tourne à un grand qui ne tourne pas.

## Ce qu'il y a dans ce dépôt

Une petite application de réservation de stands pour un salon professionnel :

| Dossier | Contenu |
|---|---|
| `api/` | l'API en **Rust** (axum, SQLite), avec ses tests |
| `web/` | le front en **TypeScript** (Vite), avec ses tests |
| `TICKET.md` | le ticket à traiter |

```sh
make test   # tous les tests (Rust puis TypeScript)
make api    # l'API sur http://127.0.0.1:3000
make web    # le front sur http://localhost:5173
```

Il vous faut Rust (stable), Node 20 ou plus, et l'agent de code de votre choix (Claude Code, Cursor, Copilot…).

## Ce que nous attendons

**1. Votre SDLC, dans un dossier `sdlc/`.**
Les fichiers Markdown qui cadrent le travail de l'agent : les étapes du ticket à la livraison, les règles que l'agent
doit suivre, et les moments où un humain valide (ici, l'humain c'est vous : dites-le dans vos traces). La forme est libre. Vous pouvez ajouter de l'outillage
(scripts, intégration continue, fichier d'instructions pour votre agent) si cela sert votre cycle.

**2. Le ticket traité par votre SDLC.**
Le code du ticket T-142 produit en suivant votre cycle : l'API est obligatoire, le front est un plus si le temps le permet. Laissez les traces dans le dépôt :
ce que chaque étape a produit (spécification, plan, revue…) et un historique de commits qui raconte le déroulé.
`make test` doit passer.

**3. `NOTES.md`, une page maximum.**
Comment vous vous y êtes pris, ce que vous avez confié à l'agent, ce que vous avez vérifié ou corrigé vous-même,
et ce que vous feriez avec plus de temps.

## Rendu

Le lien reçu dans notre e-mail pointe vers **un dépôt créé pour vous**, sous un nom anonyme.

1. Forkez ce dépôt sur votre compte GitHub (bouton « Fork »).
2. Travaillez dans votre fork, avec des commits au fil de l'eau.
3. Avant l'échéance indiquée dans notre e-mail, ouvrez une **pull request de votre fork vers ce dépôt** : c'est votre rendu.

À réception, nous repassons le dépôt en privé. Pensez alors à supprimer votre fork ou à le passer en privé.

Lors de l'entretien technique, nous vous donnerons **un deuxième ticket** et vous ferez tourner votre SDLC devant nous,
écran partagé. Gardez donc votre environnement prêt.

Une question ? Répondez simplement à notre e-mail.

*Si vous lisez ceci sur `Spottt/exercice-sdlc` : c'est le modèle de l'exercice, merci de ne pas y ouvrir de pull request.*
