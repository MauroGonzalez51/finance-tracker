#!/usr/bin/env bash

directory="$1"
shift

/usr/bin/env -C "$directory" "$@"
