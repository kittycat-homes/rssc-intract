#!/bin/sh
cargo build --release
npm i
npm run build
