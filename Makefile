docker:
	docker build -t bender .

build:
	docker run --rm --user "$(id -u)":"$(id -g)" \
		-v "$PWD":/usr/src/myapp \
		-w /usr/src/myapp \
		rust:1.23.0 cargo build --release

update:
	rustup update && cargo update

.PHONY: all
