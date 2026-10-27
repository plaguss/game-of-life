CARGO ?= cargo

.PHONY: dev build run test lint fmt fmt-check fmt-lint-test check clean

dev: run

build:
	$(CARGO) build

run:
	$(CARGO) run

test:
	$(CARGO) test

lint:
	$(CARGO) clippy --all-targets --all-features -- -D warnings

fmt:
	$(CARGO) fmt

fmt-check:
	$(CARGO) fmt -- --check

fmt-lint-test: fmt lint test

check:
	$(CARGO) check

clean:
	$(CARGO) clean
