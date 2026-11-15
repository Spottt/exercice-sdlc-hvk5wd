# Spottt — réservation de stands

- `api/` : Rust, axum + SQLite (`src/db.rs` règles et stockage, `src/lib.rs` routes et erreurs, `tests/` intégration HTTP). `web/` : TypeScript, Vite, vitest.
- Commandes : `make test` (tout), `make check` (porte verte), `make check-red` (porte rouge), `make check-scope`.
- Un ticket se traite avec le cycle `sdlc/` : `/sdlc-cadrer <id>` puis planifier, tests, implementer, revue, livrer. Lire `sdlc/README.md`.
- Règles invariantes et leurs contrôles : `sdlc/constitution.md`. Elles priment sur tout ticket.
- Messages d'erreur de l'API en français, style `{"error": "Stand inconnu"}` ; pas de `unwrap()` hors tests.
- Tests : à travers le routeur (`oneshot`), base en mémoire par test, date injectée, jamais l'horloge.
- `sdlc/`, `.claude/`, `Makefile`, `CLAUDE.md` : modifiés par un humain seulement.
- Ne commite pas : propose le bloc `git add` / `git commit` (Conventional Commits, sans scope, id du ticket en fin).
