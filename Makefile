.PHONY: run
run:
	@echo "SCHClient try to run"
	cargo run --bin main

.PHONY: test
test:
	@echo "SCHClient try to test"
	cargo test -- --nocapture