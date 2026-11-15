# Spottt — réservation de stands

- `api/` : Rust, axum + SQLite (`src/db.rs` règles et stockage, `src/lib.rs` routes et erreurs, `tests/` intégration HTTP). `web/` : TypeScript, Vite, vitest.
- Commandes : `make test` (tout), `make check` (porte verte), `make check-red` (porte rouge), `make check-scope`, `make valider` et `make bilan` (journal du ticket).
- Un ticket se traite avec le cycle `sdlc/` : `/sdlc <id>` enchaîne tout et s'arrête aux portes humaines ; `/sdlc-<étape> <id>` rejoue une étape seule. Lire `sdlc/README.md`.
- Règles invariantes et leurs contrôles : `sdlc/constitution.md`. Elles priment sur tout ticket.
- Messages d'erreur de l'API en français, style `{"error": "Stand inconnu"}` ; pas de `unwrap()` hors tests.
- Tests : à travers le routeur (`oneshot`), base en mémoire par test, date injectée, jamais l'horloge.
- `sdlc/`, `.claude/`, `Makefile`, `CLAUDE.md` : modifiés par un humain seulement.
- Commite chaque étape du cycle (Conventional Commits, sans scope, id du ticket en fin) ; ne pousse pas, n'ouvre pas de PR, ne lance jamais `make valider`.
