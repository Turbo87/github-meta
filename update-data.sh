#!/bin/bash
set -euxo pipefail

mkdir -p data
curl https://api.github.com/meta --output data/meta.json
curl https://api.github.com/meta/public_keys/secret_scanning --output data/secret-scanning.json
