all: constants_analysis intervals_analysis

constants_analysis:
	cargo build --release --bin constants_analysis
	cp ./target/release/constants_analysis ./constants_analysis

intervals_analysis:
	cargo build --release --bin intervals_analysis
	cp ./target/release/intervals_analysis ./intervals_analysis

.PHONY: clean
clean:
	rm -f constants_analysis intervals_analysis
	cargo clean
