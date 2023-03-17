#!/usr/bin/env bash

source /usr/bin/cmdline.sh

ATOMIC_SERVER=$(get_cmd atomic_server)
ATOMIC_PORT=$(get_cmd atomic_port)
/usr/bin/atomic-server --port ${ATOMIC_PORT} --server-url ${ATOMIC_SERVER} --log-level debug
