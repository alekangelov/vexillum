#!/bin/bash

set -e

echo "Starting Vexillum development environment..."
echo ""
echo "1. Starting infrastructure (podman compose)..."
make -C infra up &
INFRA_PID=$!

echo ""
echo "2. Starting backend (cargo watch)..."
(cd apps/backend && cargo watch -x run) &
BACKEND_PID=$!

echo ""
echo "3. Starting frontend (vite dev server)..."
(cd apps/frontend && pnpm dev) &
FRONTEND_PID=$!

echo ""
echo "✓ All services started!"
echo ""
echo "Frontend: http://localhost:5173"
echo "Backend: Running with auto-reload"
echo "Infrastructure: Running (PostgreSQL, PgAdmin, etc.)"
echo ""
echo "Press Ctrl+C to stop all services"

# Trap to handle graceful shutdown
cleanup() {
    echo ""
    echo "Shutting down services..."
    kill $INFRA_PID $BACKEND_PID $FRONTEND_PID 2>/dev/null || true
    wait 2>/dev/null || true
    echo "✓ All services stopped"
}

trap cleanup INT TERM

wait
