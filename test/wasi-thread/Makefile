# wasi-thread requires wasi-sdk-20.0 or later.
CC=/opt/wasi-sdk-20.0/bin/clang --sysroot=/opt/wasi-sdk-20.0/share/wasi-sysroot
CFLAGS=-g -O0 -pthread
LDFLAGS= -Wl,--import-memory -Wl,--export-memory -Wl,--max-memory=655360

TARGET=pthread_test.wasm pthread_test.aot pthread_test.xip.aot

all: $(TARGET)

.SUFFIXES: .wasm .aot .xip.aot .wat

.c.wasm:
	$(CC) -target wasm32-wasi-threads $(CFLAGS) $^ -o $@  $(LDFLAGS)

.wasm.aot:
	wamrc -o $@ $^

.wasm.xip.aot:
	wamrc   --enable-indirect-mode --disable-llvm-intrinsics -o $@ $^

.wasm.wat:
	wasm2wat --enable-all $^ -o $@

clean:
	rm $(TARGET) 2>/dev/null || true
