CARGO ?= cargo

.PHONY: dev build run jlox clox test lint fmt check clean ast fmt-lint-test

dev: run

build:
	$(CARGO) build

run:
	$(CARGO) run -- $(ARGS)

test:
	$(CARGO) test

lint:
	$(CARGO) clippy --all-targets --all-features -- -D warnings

fmt:
	$(CARGO) fmt

fmt-lint-test: fmt lint test

check:
	$(CARGO) check

clean:
	$(CARGO) clean
