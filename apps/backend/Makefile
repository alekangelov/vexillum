.PHONY: help build run dev test clean fmt lint check install-tools migrate.up migrate.down migrate.create migrate.reset migrate.status

help:
	@echo "Vexillum Backend Makefile"
	@echo ""
	@echo "Available commands:"
	@echo "  make build           - Build the project in release mode"
	@echo "  make dev             - Run the project in development mode with auto-reload"
	@echo "  make run             - Run the project"
	@echo "  make test            - Run all tests"
	@echo "  make clean           - Clean build artifacts"
	@echo "  make fmt             - Format code"
	@echo "  make lint            - Run clippy linter"
	@echo "  make check           - Check code without building"
	@echo "  make install-tools   - Install development tools (cargo-watch)"
	@echo ""
	@echo "  Migration commands:"
	@echo "  make migrate.up      - Run pending migrations"
	@echo "  make migrate.down    - Rollback the last migration"
	@echo "  make migrate.create  - Create a new migration (use NAME=your_migration_name)"
	@echo "  make migrate.reset   - Rollback all migrations"
	@echo "  make migrate.status  - Show migration status"
	@echo ""
	@echo "  make help            - Show this help message"

build:
	cargo build --release

run:
	cargo run

dev:
	@command -v cargo-watch >/dev/null 2>&1 || { echo "cargo-watch not installed. Run 'make install-tools' first."; exit 1; }
	cargo watch -x run

test:
	cargo test

clean:
	cargo clean

fmt:
	cargo fmt

lint:
	cargo clippy -- -D warnings

check:
	cargo check

install-tools:
	cargo install cargo-watch

# Migration commands
migrate.up:
	cargo run --bin migrate -- up

migrate.down:
	cargo run --bin migrate -- down

migrate.create:
	@if [ -z "$(NAME)" ]; then \
		echo "Error: NAME variable not set. Usage: make migrate.create NAME=migration_name"; \
		exit 1; \
	fi
	cargo run --bin migrate -- create "$(NAME)"

migrate.reset:
	cargo run --bin migrate -- reset

migrate.status:
	cargo run --bin migrate -- status
