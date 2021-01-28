.PHONY: gen-chart-docs lint lint-chart

lint:
	cargo fmt --all -- --check
	cargo clippy -- -D warnings

lint-chart:
	helm lint deploy/charts/vanity

gen-chart-docs:
	helm-docs deploy/charts/vanity
