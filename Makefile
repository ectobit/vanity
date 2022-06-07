.PHONY: check lint

check:
	cargo +nightly deny check
	cargo +nightly outdated --exit-code 1
	cargo +nightly udeps
	cargo +nightly audit
	cargo +nightly pants

lint:
	cargo +nightly fmt --all -- --check
	cargo +nightly clippy -- -D warnings
