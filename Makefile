LANGUAGE ?= java
LOG_LEVEL ?= debug
SCENARIOS := $(notdir $(wildcard bin/tests/scenarios/$(LANGUAGE)/*))

.PHONY: all fmt clippy test coverage run_merge_on_dir run_diff rebuild_snapshots

all: fmt clippy test

build:
	cargo build --release --all-features
	
fmt:
	cargo fmt

clippy:
	cargo clippy --workspace --all-features -- -D warnings

test:
	cargo test --workspace --all-features

coverage:
	cargo llvm-cov --workspace --all-features --html

run_merge_on_dir:
	cargo run --release -- \
		merge \
		--left-path=$(DIR)/left$(SUFFIX) \
		--base-path=$(DIR)/base$(SUFFIX) \
		--right-path=$(DIR)/right$(SUFFIX) \
		--merge-path=$(DIR)/merge.output$(SUFFIX) \
		--log-level=$(LOG_LEVEL)

run_diff:
	cargo run --release -- \
		diff \
		--left-path=$(LEFT) \
		--right-path=$(RIGHT) \
		--log-level=$(LOG_LEVEL)

rebuild_snapshots:
	@for scenario in $(SCENARIOS); do \
		$(MAKE) run_merge_on_dir DIR=bin/tests/scenarios/$(LANGUAGE)/$$scenario; \
	done
