.PHONY: all build clean clean-frontend

RENDERING_DIR=./rendering
RENDERINGWASM_DIR=./rendering_wasm
RENDERINGWASM_LIB=./rendering_wasm/target/release/librendering_wasm.rlib

SERVER_DIR=./server
SERVER_BUILD=${SERVER_DIR}/target/release/server
SUPERBUILD=./build

define print_status
	@echo ""
	@echo "////////////////////////////////////////////////////"
	@echo "// ${1}"
	@echo "////////////////////////////////////////////////////"
	@echo ""
endef


all: build

build: ${SERVER_BUILD} ${RENDERINGWASM_LIB}


${RENDERINGWASM_LIB}:
	$(call print_status, Build wasm wrapper ...)
	cd ${RENDERINGWASM_DIR} && wasm-pack build

${SERVER_BUILD}:
	$(call print_status, Build server ...)
	cd ${SERVER_DIR} && cargo build --release

${SUPERBUILD}: ${SERVER_BUILD}
	$(call print_status, Creating build directory ...)
	mkdir ${SUPERBUILD}
	cp ${SERVER_BUILD} ${SUPERBUILD}

clean-frontend: 
	$(call print_status, Cleaning up frontend [wasm] ...)
	cd ${RENDERINGWASM_DIR} && cargo clean
	rm -rf ${SUPERBUILD}

clean-server:
	$(call print_status, Cleaning up server ...)
	cd ${RENDERING_DIR} && cargo clean
	rm -rf ${RENDERINGWASM_DIR}/pkg
	cd ${SERVER_DIR} && cargo clean

clean: clean-server
	$(call print_status, Full clean up ...)
