CC = clang
CFLAGS = -Ltarget/release -lmagika_cbind

examples/magika.h:
	cargo build --release

example: examples/example.c examples/magika.h
	$(CC) $(CFLAGS) -o examples/example examples/example.c

test: example
	./examples/example ./examples/example

clean:
	rm -f examples/example
	rm -f examples/magika.h
	cargo clean

.PHONY: clean test example
