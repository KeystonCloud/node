#!/bin/sh
ipfs daemon &
cargo watch -x run
