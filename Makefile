project_name = "termchan"

.PHONY: core
core:
	@echo "$(project_name) try to run core"
	cargo run

.PHONY: cli
cli:
	@echo "$(project_name) try to run cli"
	cargo run -p $@

.PHONY: fmt
fmt:
	@echo "Formatting..."
	cargo fmt --all
	@echo "Done."

.PHONY: test
test:
	@echo "$(project_name) try to test"
	cargo test -- --nocapture

.PHONY: test-bbsmenu
test-bbsmenu:
	@echo "$(project_name) try to test bbsmenu"
	cargo test bbsmenu -- --nocapture

.PHONY: test-encoder
test-encoder:
	@echo "$(project_name) try to test encoder"
	cargo test -p core encoder -- --nocapture
