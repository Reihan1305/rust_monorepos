.PHONY: help build run-server run-worker run-scheduler migrate seed clean

help:
	@echo "Available commands:"
	@echo "  make build          - Build all binaries"
	@echo "  make run-server     - Run HTTP server"
	@echo "  make run-worker     - Run background worker"
	@echo "  make run-scheduler  - Run scheduler"
	@echo "  make migrate        - Run database migrations"
	@echo "  make seed           - Run database seeder"
	@echo "  make clean          - Clean build artifacts"

build:
	cargo build --release

run-server:
	cargo run --bin server

run-worker:
	cargo run --bin worker

run-scheduler:
	cargo run --bin scheduler

migrate:
	cargo run --bin migrator

seed:
	cargo run --bin seeder

clean:
	cargo clean
