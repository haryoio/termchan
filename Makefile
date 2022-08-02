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

.PHONY: re_migration
re_migration:
	rm -rf /var/tmp/termchan.db
	sea-orm-cli migrate down
	sea-orm-cli migrate up
	sea-orm-cli generate entity -o entity/src
	rm -rf entity/src/lib.rs
	mv entity/src/mod.rs entity/src/lib.rs

.PHONY: rmdata
rmdata:
	rm -rf /var/tmp/termchan.db

.PHONY: db
db:
	sqlite3 /var/tmp/termchan.db
