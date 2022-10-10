#!/bin/bash

set -e -o pipefail

HYPERFINE_VERSION=v1.15.0
EXPORT_FILE="${AWS_REGION}-$(date +%s)"

if ! test -x ./hyperfine; then
    curl -fsSL https://github.com/sharkdp/hyperfine/releases/download/${HYPERFINE_VERSION}/hyperfine-${HYPERFINE_VERSION}-x86_64-unknown-linux-gnu.tar.gz | \
        tar -xzvf- --strip-components=1 hyperfine-${HYPERFINE_VERSION}-x86_64-unknown-linux-gnu/hyperfine
fi

./hyperfine --warmup 10 --runs 500 --shell=none \
    "curl https://aws-region-nearby.mlafeldt.workers.dev" \
    "curl https://aws-region-nearby.deno.dev" \
    --export-json "${EXPORT_FILE}.json" --export-csv "${EXPORT_FILE}.csv"

aws s3 cp "${EXPORT_FILE}.json" s3://aws-region-nearby-benchmarks
aws s3 cp "${EXPORT_FILE}.csv"  s3://aws-region-nearby-benchmarks
