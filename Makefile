plugins := $(wildcard target/debug/*.so)
releaseplugins := $(wildcard target/release/*.so)

build:
	cargo build --all
	for path in $(plugins); do \
		cp $$path plugins/; \
	done

build-release:
	cargo build --all --release
	for path in $(releaseplugins); do \
		cp $$path plugins/; \
	done

run: build
	cargo run

run-release: build-release
	cargo run --release