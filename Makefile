PROJECT_NAME = "termchan"
DEPENDENICIES = "cargo-watch"

.PHONY: core
core:
	@echo "$(PROJECT_NAME) try to run core"
	cargo run


.PHONY: cli
cli:
	@echo "$(PROJECT_NAME) try to run cli"
	cargo run -p $@

.PHONY: cli_b
cli_b:
	@echo "$(PROJECT_NAME) try to run cli_b"
	cargo run -p cli --bin board_list

.PHONY: test-cli-main
test-cli-main:
	@echo "$(PROJECT_NAME) try to run test-cli-main"
	cargo test -p cli test-cli-main

.PHONY: test
test:
	@echo "$(PROJECT_NAME) try to test"
	cargo test -- --nocapture

.PHONY: test-bbsmenu
test-bbsmenu:
	@echo "$(PROJECT_NAME) try to test bbsmenu"
	cargo test bbsmenu -- --nocapture

.PHONY: test-encoder
test-encoder:
	@echo "$(PROJECT_NAME) try to test encoder"
	cargo test encoder -- --nocapture

.PHONY: test-config
test-config:
	@echo "$(PROJECT_NAME) try to test config"
	cargo test config -- --nocapture

.PHONY: fmt
fmt:
	cargo fmt
