
TEST1=test1.wasm test1.wat test1.aot test1.xip.aot
TEST2=test2.wasm test2.wat test2.aot test2.xip.aot

all: $(TEST1) $(TEST2) build-wasi-thread

.SUFFIXES: .rs .wasm .aot .xip.aot .wat

.c.wasm:
	/opt/wasi-sdk-20.0/bin/clang -g -O0 -o $@ $^

.rs.wasm:
	rustc --target wasm32-wasi $^  -o $@

.wasm.aot:
	wamrc -o $@ $^

.wasm.xip.aot:
	wamrc   --enable-indirect-mode --disable-llvm-intrinsics -o $@ $^

.wasm.wat:
	wasm2wat --enable-all $^ -o $@

build-wasi-thread:
	make -C wasi-thread

clean:
	rm $(TEST1) 2>/dev/null || true
	rm $(TEST2) 2>/dev/null || true
	make clean -C wasi-thread || true
