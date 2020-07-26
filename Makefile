.PHONY: all build clean

RENDERING_DIR=./rendering
RENDERING_LIB=${RENDERING_DIR}/target/release
RENDERINGWASM_DIR=./rendering_wasm
RENDERINGWASM_LIB=./rendering_wasm/target/release

FRONTEND_DIR=./frontend
RENDERINGNPM_MOD=${FRONTEND_DIR}/node_modules/rendering_wasm
FRONTEND_BUILD=${FRONTEND_DIR}/dist

BACKEND_DIR=./backend

SERVER_DIR=./server
SERVER_BUILD=${SERVER_DIR}/target/release

define print_status
	@echo ""
	@echo "////////////////////////////////////////////////////"
	@echo "// ${1}"
	@echo "////////////////////////////////////////////////////"
	@echo ""
endef


all: build

build:  ${FRONTEND_BUILD} ${SERVER_BUILD}

${RENDERING_LIB}:
	$(call print_status, Build rendering library ...)
	cd ${RENDERING_DIR} && cargo build --release

${RENDERINGWASM_LIB}: ${RENDERING_LIB}
	$(call print_status, Build wasm wrapper ...)
	cd ${RENDERINGWASM_DIR} && wasm-pack build

${FRONTEND_BUILD}: ${RENDERINGWASM_LIB}
	$(call print_status, Build frontend ...)
	cp -r ${RENDERINGWASM_DIR}/pkg ${RENDERINGNPM_MOD}
	cd ${FRONTEND_DIR} && npm install && npm run prod
	cp -r ${FRONTEND_BUILD} ${BACKEND_DIR}

${SERVER_BUILD}: ${RENDERING_LIB}
	$(call print_status, Build server ...)
	cd ${SERVER_DIR} && cargo build --release

clean:
	$(call print_status, Cleaning up ...)
	cd ${RENDERING_DIR} && cargo clean
	cd ${RENDERINGWASM_DIR} && cargo clean
	cd ${SERVER_DIR} && cargo clean
	rm -rf ${RENDERINGNPM_MOD}
	rm -rf ${FRONTEND_BUILD}
