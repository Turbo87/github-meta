#!/bin/bash
set -euxo pipefail

mkdir -p data

curl https://api.github.com/meta --output data/meta.json
jq --monochrome-output '.' data/meta.json > data/meta.json.tmp
mv data/meta.json.tmp data/meta.json

curl https://api.github.com/meta/public_keys/secret_scanning --output data/secret-scanning.json
jq --monochrome-output '.' data/secret-scanning.json > data/secret-scanning.json.tmp
mv data/secret-scanning.json.tmp data/secret-scanning.json
