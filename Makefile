PROJECT_NAME = "termchan"
DEPENDENICIES = "cargo-watch"
ERROR_LOG = "termchan.log"


.PHONY: core
core:
	@echo "$(PROJECT_NAME) try to run core"
	cargo run

.PHONY: client
client:
	@echo "$(PROJECT_NAME) try to run cli"

	cargo run -p $@ 2> $(ERROR_LOG)

.PHONY: test
test:
	@echo "$(PROJECT_NAME) try to test"
	cargo test -- --nocapture

.PHONY: fmt
fmt:
	cargo fmt


.PHONY: debug
debug:
	export RUST_BACKTRACE=1
