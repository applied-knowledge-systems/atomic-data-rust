#!/usr/bin/env bash

source /usr/local/cmdline.sh

ATOMIC_SERVER=get_cmd hostname
/usr/bin/atomic-server --server-url ${ATOMIC_SERVER} --data-dir /atomic/db --config-dir /atomic/config --log-level debug