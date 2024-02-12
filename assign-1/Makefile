CONST_SRC = ./src/bin/constants_analysis.rs ./src/cfg.rs ./src/lir.rs ./src/store.rs ./src/utils.rs ./src/abs/domain.rs ./src/abs/semantics.rs ./src/abs/execution.rs
INTER_SRC = ./src/bin/intervals_analysis.rs ./src/cfg.rs ./src/lir.rs ./src/store.rs ./src/utils.rs ./src/abs/domain.rs ./src/abs/semantics.rs ./src/abs/execution.rs

all: constants_analysis intervals_analysis

debug: constants_analysis_debug intervals_analysis_debug

test: test_constants_analysis test_intervals_analysis

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

test_constants_analysis: constants_analysis
	@echo "Running constants analysis on test files from ./demos/"
	@bash run-demo-const.sh
	@echo "Comparing the output with the expected output"
	@bash diff-demo-const.sh
	@echo ""

test_intervals_analysis: intervals_analysis
	@echo "Running intervals analysis on test files from ./demos/"
	@bash run-demo-inter.sh
	@echo "Comparing the output with the expected output"
	@bash diff-demo-inter.sh
	@echo ""

package:
	mkdir -p assign-1
	cp -r src/ assign-1/
	cp Cargo.toml assign-1/
	cp Cargo.lock assign-1/
	cp Makefile assign-1/
	cp run-constants-analysis.sh assign-1/
	cp run-intervals-analysis.sh assign-1/
	cp build-analyses.sh assign-1/
	cd assign-1 && zip -r ../assign-1.zip *

.PHONY: clean
clean:
	rm -f constants_analysis intervals_analysis *.out.tmp.*
	rm -rf assign-1 assign-1.zip
	cargo clean
