SCENARIOS=$(shell ls bin/tests/scenarios)
LANGUAGE=java
SUFFIX=.java
LOG_LEVEL=debug

run_merge_on_dir:
	cargo run -- merge --left-path=$(DIR)/left$(SUFFIX) --base-path=$(DIR)/base$(SUFFIX) --right-path=$(DIR)/right$(SUFFIX) --merge-path=$(DIR)/merge.output$(SUFFIX) --log-level=$(LOG_LEVEL) --language=$(LANGUAGE)

run_diff:
	cargo run -- diff --left-path=$(LEFT_PATH) --right-path=$(RIGHT_PATH) --language=$(LANGUAGE) --log-level=$(LOG_LEVEL)

rebuild_snapshots:
	for SCENARIO in $(SCENARIOS); do \
		make run_merge_on_dir DIR=bin/tests/scenarios/$$SCENARIO; \
	done