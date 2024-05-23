
run:
	docker compose down
	docker-compose build --no-cache
	docker compose up -d
	docker compose logs --tail 10 --follow

build:
	docker compose build
