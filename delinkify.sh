#!/bin/bash
set -euo pipefail
IFS=$'\n\t'

if [[ $# -ne 1 ]]; then
    echo "usage: $0 <dir>" >&2
    exit 1
fi
dir="$1"

for file in $(find "${dir}" -type l); do
    path="$(readlink -f "${file}")"
    rm -f "${file}"
    cp -r "${path}" "${file}"
    echo "delinkified ${file}"
done
