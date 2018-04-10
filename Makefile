include/avro_rs.h: $(shell find src -type f -name "*.rs")
	RUSTUP_TOOLCHAIN=nightly cbindgen -v -c cbindgen.toml . -o $@

.PHONY: build
build: include/avro_rs.h
	cargo build

.PHONY: test
test: include/avro_rs.h
	cargo test

.PHONY: release
release: include/avro_rs.h
	cargo build --release

.PHONY: fmt
fmt:
	cargo +nightly fmt

.PHONY: clean
clean:
	cargo clean
	rm -rf include
