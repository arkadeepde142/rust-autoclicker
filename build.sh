#!/bin/bash
export MACOSX_DEPLOYMENT_TARGET=10.7;
cargo build --release;
mkdir release/temp;
cp target/release/autoclicker release/temp/;
cp README.md LICENSE release/temp/;
tar czvf release/autoclicker.tar.gz release/temp/;
rm -rf release/temp;