project_name = "termchan"

.PHONY: core
core:
	@echo "$(project_name) try to run core"
	cargo run -p $@

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
