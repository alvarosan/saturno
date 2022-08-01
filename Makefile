.PHONY: all build clean

RENDERINGWASM_DIR=./rendering_wasm
RENDERINGWASM_LIB=./rendering_wasm/target/debug/librendering_wasm.so

RENDERING_LIB=./rendering/target/debug/librendering.rlib

define print_status
	@echo ""
	@echo "////////////////////////////////////////////////////"
	@echo "// ${1}"
	@echo "////////////////////////////////////////////////////"
	@echo ""
endef


all: build

build: ${RENDERING_LIB} ${RENDERINGWASM_LIB}

${RENDERING_LIB}:
	$(call print_status, Build rendering ...)
	cargo build

${RENDERINGWASM_LIB}:
	$(call print_status, Build wasm wrapper ...)
	cd ${RENDERINGWASM_DIR} && wasm-pack build

clean-workspace:
	$(call print_status, Cleaning up cargo workspace ...)
	cargo clean
	rm ${RENDERINGWASM_LIB}

clean: clean-workspace
	$(call print_status, Full clean up ...)
