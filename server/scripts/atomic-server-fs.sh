#!/usr/bin/env bash

source /usr/local/cmdline.sh

ATOMIC_DOMAIN=get_cmd hostname
/usr/bin/atomic-server --domain ${ATOMIC_DOMAIN} --data-dir /atomic/db --config-dir /atomic/config --log-level debug