.PHONY: check lint

lint:
	cargo fmt --all -- --check
	cargo clippy -- -D warnings

check:
	cargo deny check
	cargo outdated --exit-code 1
	cargo +nightly udeps
	cargo audit
	cargo pants
