run:
	cargo run

build:
	cargo build --release

install:
	cargo build --release
	cp target/release/donut ~/.local/bin/donut

uninstall:
	rm ~/.local/bin/donut

help:
	@echo "Usage:"
	@echo " * make run: run in debug"
	@echo " * make install: install donut to ~/.local/bin/"
	@echo " * make uninstall: uninstall donut"
