dev:
	watchexec --exts rs,tera,hbs --restart "cargo run"

doc:
	cargo doc --open

docker:
	docker build -t bender .

build:
	docker run --rm --user "$(id -u)":"$(id -g)" \
		-v "$PWD":/usr/src/myapp \
		-w /usr/src/myapp \
		rust:1.23.0 cargo build --release

setup:
	rustup update && cargo update
	cargo install watchexec
	cargo update

.PHONY: all
