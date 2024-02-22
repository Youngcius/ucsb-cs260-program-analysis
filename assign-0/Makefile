all:
	cargo build --release
	cp target/release/cs260 ./cs260
	cp target/release/parse_json ./parse_json

test:
	./cs260

.PHONY: clean
clean:
	rm -f cs260
	cargo clean
