#!/usr/bin/env sh

# RUST_BACKTRACE=1 \
    RUST_LOG=client=debug,server=debug,shared=debug,warn \
	cargo run \
	--package client \
	--bin client
