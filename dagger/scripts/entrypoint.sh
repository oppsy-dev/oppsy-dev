#!/bin/sh
set -e

cd /core-db
atlas migrate apply --url sqlite:///data/oppsy.db --env sqlite --config file:///core-db/atlas.hcl
exec /usr/local/bin/service
