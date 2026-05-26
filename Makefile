.PHONY: up down logs build test migrate clean

# Start all services in detached mode
up:
	docker-compose up -d

# Stop all services
down:
	docker-compose down

# Show logs (follow mode)
logs:
	docker-compose logs -f

# Show logs for specific service
logs-backend:
	docker-compose logs -f backend

logs-frontend:
	docker-compose logs -f frontend

logs-postgres:
	docker-compose logs -f postgres

# Build all services
build:
	docker-compose build

# Build without cache
build-no-cache:
	docker-compose build --no-cache

# Run backend tests
test:
	docker-compose exec backend cargo test

# Run database migrations
migrate:
	docker-compose exec backend cargo run -- migrate

# Open psql shell
db-shell:
	docker-compose exec postgres psql -U zyblog -d zyblog

# Clean up volumes and containers
clean:
	docker-compose down -v
	docker system prune -f

# Restart services
restart:
	docker-compose restart

# Show running containers
ps:
	docker-compose ps

# Copy .env.example to .env if .env doesn't exist
env:
	@test -f .env || cp .env.example .env
	@echo "Environment file ready. Edit .env to customize."
