#!/bin/bash

set -e

echo "Starting raftkv volume: ${VOLUME_ID}"

exec raftkv-volume server \
    --id "${VOLUME_ID}" \
    --bind "${HTTP_BIND}" \
    --grpc "${GRPC_BIND}" \
    --data "${DATA_PATH}" \
    --wal ${WAL_PATH}" \
    --coordinators "${COORDINATORS}" \