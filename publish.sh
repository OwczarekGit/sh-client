#!/bin/bash

arch="x86_64-unknown-linux-gnu"

cargo build --release --target $arch
cp "./target/$arch/release/sh-client" "./sh-client-linux-x64"
