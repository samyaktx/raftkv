#!/bin/bash
set -e

echo "Starting raft-kv coordinator: ${NODE_ID}"

exec raft-kv-coord server \
    --id "${NODE_ID}" \
    --bind "${HTTP_BIND}" \
    --grpc "${GRPC_BIND}" \
    --db "${DB_PATH}" \
    --peers "${PEERS}" \
    --replicas "${REPLICAS}"