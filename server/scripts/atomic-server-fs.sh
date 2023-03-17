#!/usr/bin/env bash

source /usr/bin/cmdline.sh

ATOMIC_DOMAIN=$(get_cmd hostname)
/usr/bin/atomic-server --port 8080 --domain ${ATOMIC_DOMAIN} --data-dir /atomic/db --config-dir /atomic/config --log-level debug
