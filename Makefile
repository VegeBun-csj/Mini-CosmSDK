run-debug:
	RUST_LOG=DEBUG cargo run -- run --verbose

run:
	cargo run -- run

test:
	cargo test

install:
	cargo install --path ./mini-cosm

init:
	./mini-cosm/scripts/init.sh

tendermint-start:
	tendermint start --home ~/.mini-cosm

.PHONY: run run-debug test install init tendermint-start