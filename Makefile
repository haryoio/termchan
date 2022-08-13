PROJECT_NAME = "termchan"
DEPENDENICIES = "cargo-watch"
ERROR_LOG =termchan.log
DB_FILE=termchan.db
DB_PATH=/var/tmp/


.PHONY: build
build:
	cargo build --release --bin termchan

.PHONY: deps
deps:
	cargo install sea-orm-cli

.PHONY: client
client:
	clear
	cargo run -p termchan-tui 2> $(ERROR_LOG)
	clear

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: debug
debug:
	export RUST_BACKTRACE=1

.PHONY: log
log:
	tail -f $(ERROR_LOG)


### DB ###

.PHONY: re_migrate
re_migrate:
	sea-orm-cli migrate down
	sea-orm-cli migrate up

.PHONY: migrateup
migrateup:
	sea-orm-cli migrate up

.PHONY: rmdb
rmdb:
	rm -rf $(DB_PATH)$(DB_FILE)*

.PHONY: db
db:
	sqlite3 $(DB_PATH)$(DB_FILE)
