dev:
	cargo build

release:
	cargo build --release

test:
	cargo test

doc:
	cargo doc --no-deps