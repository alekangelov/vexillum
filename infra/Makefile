.PHONY: up down logs clean reset ps stop help

help:
	@echo "Vexillum Infrastructure Commands"
	@echo "================================"
	@echo "make up       - Start all services"
	@echo "make down     - Stop all services"
	@echo "make stop     - Stop all services (alias for down)"
	@echo "make logs     - View logs from all services"
	@echo "make ps       - List running containers"
	@echo "make clean    - Remove volumes and stopped containers"
	@echo "make reset    - Stop services and remove volumes"
	@echo "make build    - Build/rebuild images"

up:
	podman compose up -d
	@echo "✓ Services started"

down:
	podman compose down
	@echo "✓ Services stopped"

stop: down

logs:
	podman compose logs -f

ps:
	podman compose ps

clean:
	podman compose down -v
	@echo "✓ Volumes removed"

reset: clean
	@echo "✓ Infrastructure reset"

build:
	podman compose build
	@echo "✓ Images built"
