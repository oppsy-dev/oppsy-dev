#!/bin/sh
set -e


cd /data/core-db && atlas migrate apply --url sqlite://oppsy.db --env sqlite --config file://atlas.hcl
exec /usr/local/bin/service
