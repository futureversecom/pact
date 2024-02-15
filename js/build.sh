#!/usr/bin/env bash

set -ex

# Check if jq is installed
if ! [ -x "$(command -v jq)" ]; then
    echo "jq is not installed" >& 2
    exit 1
fi

# Clean previous packages
if [ -d "pact-web" ]; then
    rm -rf pact-web
fi

if [ -d "pact-nodejs" ]; then
    rm -rf pact-nodejs
fi

# build for web js target
rustup run nightly wasm-pack build --target web --scope therootnetwork --out-name pact-web --release --out-dir pact-web
# modify package.json for web
jq '.name="@therootnetwork/pact-web"' pact-web/package.json > temp.json && mv temp.json pact-web/package.json

# build for nodejs target
rustup run nightly wasm-pack build --target nodejs --scope therootnetwork --out-name pact-nodejs --release --out-dir pact-nodejs
# modify package.json for nodejs
jq '.name="@therootnetwork/pact-nodejs"' pact-nodejs/package.json > temp.json && mv temp.json pact-nodejs/package.json
