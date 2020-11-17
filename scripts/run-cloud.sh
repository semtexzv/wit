#!/bin/bash



cargo run -- control -l 127.0.0.1:9090
sleep 1
cargo run -- router -l 127.0.0.1:9091 -c 127.0.0.1:9090