CONST_SRC = ./src/bin/constants_analysis.rs ./src/abs.rs ./src/cfg.rs ./src/lir.rs ./src/store.rs ./src/utils.rs
INTER_SRC = ./src/bin/intervals_analysis.rs ./src/abs.rs ./src/cfg.rs ./src/lir.rs ./src/store.rs ./src/utils.rs

all: constants_analysis intervals_analysis

debug: constants_analysis_debug intervals_analysis_debug

constants_analysis: $(CONST_SRC)
	cargo build --release --bin constants_analysis
	cp ./target/release/constants_analysis ./constants_analysis

intervals_analysis: $(INTER_SRC)
	cargo build --release --bin intervals_analysis
	cp ./target/release/intervals_analysis ./intervals_analysis

constants_analysis_debug: $(CONST_SRC)
	cargo build --bin constants_analysis
	cp ./target/debug/constants_analysis ./constants_analysis

intervals_analysis_debug: $(INTER_SRC)
	cargo build --bin intervals_analysis
	cp ./target/debug/intervals_analysis ./intervals_analysis

.PHONY: clean
clean:
	rm -f constants_analysis intervals_analysis
	cargo clean
