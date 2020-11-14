#!/bin/bash

set -exuo

ID=$(curl -v -XPOST localhost:9091/functions --data-binary @./target/wasm32-unknown-unknown/debug/capitalize.wasm | jq -r '.id')

curl -v -XPOST -H "Content-Type: application/json" localhost:9091/rules -d @- <<EOF
{ "func": "$ID", "spec": "/http/com/semtexzv" }
EOF

