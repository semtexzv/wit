#!/bin/bash

set -exuo

ID=$(curl -v -XPOST localhost:9090/functions --data-binary @./target/wasm32-unknown-unknown/debug/capitalize.wasm | jq -r '.id')

curl -v -XPOST -H "Content-Type: application/json" localhost:9090/assignments -d @- <<EOF
{ "func": "$ID", "spec": "/http/com/semtexzv"}
EOF

