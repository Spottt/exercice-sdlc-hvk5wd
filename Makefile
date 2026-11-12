.PHONY: test api web install

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
