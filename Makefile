ROOT_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))

# PHONY means that it doesn't correspond to a file; it always runs the build commands.

.PHONY: build
build: build-static

.PHONY: run
run: run-static

.PHONY: build-static
build-static:
	@cd bsv-go && cargo build --release
	@cp bsv-go/target/release/libbsv_go.a lib/
	go build gobsv.go

.PHONY: run-static
run-static: build-static
	@./gobsv

.PHONY: clean
clean:
	rm -rf gobsv lib/libhello.a bsv-go/target