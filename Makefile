CARGO ?= cargo
VHS ?= vhs

.PHONY: dev build run test lint fmt fmt-check fmt-lint-test check release clean

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

release:
	@test -z "$$(git status --porcelain)" || { printf '%s\n' 'Commit all changes before releasing.'; exit 1; }
	@command -v $(VHS) >/dev/null || { printf '%s\n' 'VHS is required to generate the demo.'; exit 1; }
	$(CARGO) fmt -- --check
	$(CARGO) clippy --locked --all-targets --all-features -- -D warnings
	$(CARGO) test --locked
	$(CARGO) build --release --locked
	$(VHS) examples/demo.tape
	$(CARGO) publish --dry-run --locked
	$(CARGO) publish --locked

clean:
	$(CARGO) clean
