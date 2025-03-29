default:
  @just --list

# --- server ---

build-server:
  (cd ./server && cargo build)

check-server:
  (cd ./server && cargo check)

# ---- web ----

build-web:
  (cd ./web-client && npm run build)

check-web:
  (cd ./web-client && npm run check)

init-web:
  (cd ./web-client && npm install)

# ---- all ----

init: init-web

build: build-server build-web

check: check-server check-web