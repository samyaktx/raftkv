#!/bin/bash
set -e

echo "Starting raftkv coordinator: ${NODE_ID}"

exec raftkv-coord server \
    --id "${NODE_ID}" \
    --bind "${HTTP_BIND}" \
    --grpc "${GRPC_BIND}" \
    --db "${DB_PATH}" \
    --peers "${PEERS}" \
    --replicas "${REPLICAS}"