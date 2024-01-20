# See https://github.com/bytecodealliance/wasm-micro-runtime/blob/main/doc/build_wasm_app.md
# wasi-thread requires wasi-sdk-20.0 or later.
CC=/opt/wasi-sdk-20.0/bin/clang --sysroot=/opt/wasi-sdk-20.0/share/wasi-sysroot
CFLAGS=-g -O0 -pthread

# Note --max-memory is used to specifiy the maximum size of the linear memory
# must be a multiple of 65536
WASI_THREAD_OPT= -Wl,--import-memory \
		 -Wl,--export-memory \
		 -Wl,--max-memory=1048576 \
		 -pthread

TARGET=pthread_test.wasm pthread_test.aot pthread_test.xip.aot

all: $(TARGET)

.SUFFIXES: .wasm .aot .xip.aot .wat

.c.wasm:
	$(CC) \
		-target wasm32-wasi-threads \
		-z stack-size=8192 			\
		-Wl,--initial-memory=65536 \
		$(WASI_THREAD_OPT) \
		$^ -o $@


.wasm.aot:
	wamrc -o $@ $^

.wasm.xip.aot:
	wamrc   --enable-indirect-mode --disable-llvm-intrinsics -o $@ $^

.wasm.wat:
	wasm2wat --enable-all $^ -o $@

clean:
	rm $(TARGET) 2>/dev/null || true
