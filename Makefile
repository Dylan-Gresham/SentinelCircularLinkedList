all: clean build check run

.PHONY: clean
clean:
	@cargo clean

build:
	@cargo build --release

check:
	@cargo test --no-fail-fast --release

run:
	@cargo -q run --release

.PHONY: install-deps
install-deps:
	sudo apt-get update -y
	sudo apt-get install -y libio-socket-ssl-perl libmime-tools-perl