run:
	source .env && \
	cargo fmt && cargo check && cargo run
build:
	docker compose down
	docker-compose build --no-cache
	docker compose up -d
	docker compose logs --tail 10 --follow
makedb:
	sqlite3 sqlite.db "VACUUM;"
