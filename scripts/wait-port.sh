#!/bin/bash
#
# wait-port.sh PORT
#

sleep 1
echo -n "Waiting port $1 ... "; for _ in `seq 1 60`; do echo -n .; sleep 0.5; nc -z localhost $1 && echo " Open." && exit ; done; echo " Timeout!" >&2; exit 1
