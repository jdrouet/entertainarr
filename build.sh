#!/bin/bash

cd packages/web
trunk build --release
cd ../..
cp -r packages/web/dist/* packages/server/public/
cargo build --release --package entertainarr-server
