#!/usr/bin/env bash

set -eux

rm -rf ~/.mini-cosm
cargo run -- init test
