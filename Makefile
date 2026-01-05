.PHONY: help dev

help:
	@echo "Vexillum Development Commands"
	@echo "=============================="
	@echo "make dev  - Start infra, backend, and frontend in development mode"
	@echo "make help - Show this help message"

dev:
	@bash scripts/dev.sh
