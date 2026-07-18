SCENARIOS=$(shell ls bin/tests/scenarios/java)
LANGUAGE=java
LOG_LEVEL=debug

run_merge_on_dir:
	cargo run --release -- merge --left-path=$(DIR)/left$(SUFFIX) --base-path=$(DIR)/base$(SUFFIX) --right-path=$(DIR)/right$(SUFFIX) --merge-path=$(DIR)/merge$(SUFFIX) --log-level=$(LOG_LEVEL)

run_diff:
	cargo run --release -- diff --left-path=$(LEFT) --right-path=$(RIGHT) --log-level=$(LOG_LEVEL)

rebuild_snapshots:
	for SCENARIO in $(SCENARIOS); do \
		make run_merge_on_dir DIR=bin/tests/scenarios/java/$$SCENARIO; \
	done