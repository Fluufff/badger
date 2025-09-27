#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail
IFS=$'\n\t\v'
cd `dirname "${BASH_SOURCE[0]:-$0}"`

if [[ ! -f .db.env ]]; then
    echo -n "MARIADB_ROOT_PASSWORD=" >> .db.env
    head -c 100 /dev/urandom | tr -dc A-Za-z0-9 | head -c 64 >> .db.env
    echo "" >> .db.env
fi
if [[ ! -f .jwt.env ]]; then
    echo -n "JWT_SECRET=" >> .jwt.env
    head -c 100 /dev/urandom | tr -dc A-Za-z0-9 | head -c 64 >> .jwt.env
    echo "" >> .jwt.env
fi

echo "init done"