#!/bin/bash

set -euo pipefail

WORKER_NAME="engine-worker"
STAGING_DIR="${TRUNK_STAGING_DIR:?TRUNK_STAGING_DIR is not set}"

WORKER_JS="${STAGING_DIR}/${WORKER_NAME}.js"
WORKER_WASM="${STAGING_DIR}/${WORKER_NAME}_bg.wasm"
INDEX_HTML="${STAGING_DIR}/index.html"

if [[ ! -f "$WORKER_JS" || ! -f "$WORKER_WASM" ]]; then
  echo "Error: Expected worker artifacts not found in ${STAGING_DIR}" >&2
  exit 1
fi

# Compute a 16-char SHA-256 content hash across both the WASM binary and JS glue
JS_HASH=$(cat "$WORKER_JS" | sha256sum | cut -c1-16)
WASM_HASH=$(cat "$WORKER_WASM" | sha256sum | cut -c1-16)

HASHED_JS_NAME="${WORKER_NAME}-${JS_HASH}.js"
HASHED_WASM_NAME="${WORKER_NAME}-${WASM_HASH}_bg.wasm"

# Update any hardcoded .wasm references inside the JS glue
sed -i "s|${WORKER_NAME}_bg\.wasm|${HASHED_WASM_NAME}|g" "$WORKER_JS"
sed -i "s|${WORKER_NAME}\.js|${HASHED_JS_NAME}|g" "$WORKER_JS"

# Rename both files with the matching content hash
mv "$WORKER_WASM" "${STAGING_DIR}/${HASHED_WASM_NAME}"
mv "$WORKER_JS" "${STAGING_DIR}/${HASHED_JS_NAME}"

# Inject the final cache-busted worker URL into index.html
sed -i "s|__ENGINE_WORKER_JS_URL__|${HASHED_JS_NAME}|g" "$INDEX_HTML"
sed -i "s|__ENGINE_WORKER_WASM_URL__|${HASHED_WASM_NAME}|g" "$INDEX_HTML"

echo "Cache-busted worker files generated: ${HASHED_JS_NAME} and ${HASHED_WASM_NAME}"
