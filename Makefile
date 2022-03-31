.PHONY: run
run:
	cargo run --bin main

.PHONY: clia
clia:
	cargo run -p cli

.PHONY: fmt
fmt:
	@echo "Formatting..."
	cargo fmt --all
	@echo "Done."

.PHONY: test
test:
	@echo "SCHClient try to test"
	cargo test -- --nocapture
