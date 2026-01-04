.PHONY: help dev

help:
	@echo "Vexillum Development Commands"
	@echo "=============================="
	@echo "make dev  - Start infra, backend, and frontend in development mode"
	@echo "make help - Show this help message"

dev:
	@echo "Starting Vexillum development environment..."
	@echo ""
	@echo "1. Starting infrastructure (podman compose)..."
	@make -C infra up
	@echo ""
	@echo "2. Starting backend (cargo watch)..."
	@cd apps/backend && cargo watch -x run &
	@echo ""
	@echo "3. Starting frontend (vite dev server)..."
	@cd apps/frontend && pnpm dev &
	@echo ""
	@echo "✓ All services started!"
	@echo ""
	@echo "Frontend: http://localhost:5173"
	@echo "Backend: Running with auto-reload"
	@echo "Infrastructure: Running (PostgreSQL, PgAdmin, etc.)"
	@echo ""
	@echo "Press Ctrl+C to stop all services"
	@wait
