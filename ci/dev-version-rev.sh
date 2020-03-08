#!/bin/bash

current="0.9.3-dev"
commit="$(git rev-parse --short HEAD)";
next="${current}+${commit}"
echo $current
echo $next

sed "s/$current/$next/" Cargo.toml > Cargo.toml

# cargo check
