plugins := $(wildcard target/debug/*.so)

build:
	cargo build --all
	for path in $(plugins); do \
		cp $$path plugins/; \
	done

run: build
	cargo run