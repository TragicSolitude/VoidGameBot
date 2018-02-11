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
	LD_LIBRARY_PATH=$$(rustc --print sysroot)/lib:$$LD_LIBRARY_PATH
	cargo +stable run

run-release: build-release
	LD_LIBRARY_PATH=$$(rustc --print sysroot)/lib:$$LD_LIBRARY_PATH
	cargo +stable run --release