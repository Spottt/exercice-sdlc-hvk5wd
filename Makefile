.PHONY: test api web install check check-red check-scope

# Tous les tests (API Rust puis front TypeScript).
test:
	cd api && cargo test
	cd web && npm install --silent && npm test

# L'API sur http://127.0.0.1:3000
api:
	cd api && cargo run

# Le front sur http://localhost:5173 (l'API doit tourner)
web:
	cd web && npm install --silent && npm run dev

# Ticket actif : 2e mot de .sdlc-step (écrit par les commandes /sdlc-*), ou make <cible> T=T-xxx
T ?= $(word 2,$(shell cat .sdlc-step 2>/dev/null))
SPEC = docs/tickets/$(T)/spec.md
# Base du ticket : le commit qui a ajouté sa spec ; HEAD tant qu'elle n'est pas commitée
BASE = $(or $(shell git log --diff-filter=A --format=%H -- $(SPEC) | tail -1),HEAD)
CHANGED = { git diff --name-only $(BASE); git ls-files --others --exclude-standard; } | sort -u
TESTS_RE = ^api/tests/.+\.rs$$|\.test\.ts$$

# P3 · Vert
check: check-scope
	cd api && cargo fmt --check
	cd api && cargo clippy --all-targets -- -D warnings
	cd api && cargo test
	cd web && npm install --silent && npm test
	@git diff --quiet HEAD -- api/tests '*.test.ts' && test -z "$$(git ls-files --others --exclude-standard api/tests '*.test.ts')" \
	  || { echo "check : tests gelés modifiés depuis le dernier commit"; exit 1; }
	@echo "check OK"

# P2 · Rouge
check-red: check-scope
	@! grep -rnE 'SystemTime::now|Local::now|Utc::now|sleep\(' api/tests || { echo "check-red : horloge ou sleep dans api/tests"; exit 1; }
	@out=$$($(CHANGED) | grep -v '^docs/tickets/$(T)/' | grep -vE '$(TESTS_RE)'); \
	test -z "$$out" || { echo "check-red : seuls des tests peuvent bouger à cette étape :"; echo "$$out"; exit 1; }
	cd api && cargo test --no-run
	@new=$$($(CHANGED) | grep -E '$(TESTS_RE)'); \
	test -n "$$new" || { echo "check-red : aucun test nouveau"; exit 1; }; \
	old=""; for f in api/tests/*.rs; do echo "$$new" | grep -qxF "$$f" || old="$$old --test $$(basename $$f .rs)"; done; \
	(cd api && cargo test --lib --bins $$old) || { echo "check-red : un ancien test casse"; exit 1; }; \
	for f in $$new; do out=""; case $$f in \
	  api/*) out=$$(cd api && cargo test --test $$(basename $$f .rs) 2>&1); echo "$$out" | grep -q 'test result: FAILED. 0 passed' ;; \
	  web/*) ! (cd web && npx vitest run $${f#web/}) ;; \
	esac || { echo "$$out"; echo "check-red : $$f doit être entièrement rouge"; exit 1; }; \
	echo "$$out" | grep -E '^test |test result'; done; \
	echo "check-red OK : nouveaux tests rouges, anciens verts"

# N'écrire que dans le bloc Fichiers de la spec (plus docs/tickets/<T>/ et NOTES.md)
check-scope:
	@test -n "$(T)" || { echo "check-scope : ticket inconnu, lancer une commande /sdlc-* ou make $@ T=T-xxx"; exit 1; }
	@test -f $(SPEC) || { echo "check-scope : $(SPEC) absent"; exit 1; }
	@out=$$($(CHANGED) | awk -v t="docs/tickets/$(T)/" \
	  'FNR == NR { if (/^```/) f = /^```files/; else if (f && NF) ok[$$1] = 1; next } \
	   !($$0 in ok) && index($$0, t) != 1 && $$0 != "NOTES.md"' $(SPEC) -); \
	test -z "$$out" || { echo "check-scope : hors du bloc Fichiers de $(SPEC) :"; echo "$$out"; exit 1; }
	@echo "check-scope OK ($(T), base $(BASE))"
