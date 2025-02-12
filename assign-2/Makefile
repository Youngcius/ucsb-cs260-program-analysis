RDEF_SRC = ./src/bin/rdef.rs ./src/cfg.rs ./src/lir.rs ./src/store.rs ./src/utils.rs ./src/abs/domain.rs ./src/abs/semantics.rs ./src/abs/execution.rs
CTRL_SRC = ./src/bin/ctrl.rs ./src/cfg.rs ./src/lir.rs ./src/store.rs ./src/utils.rs ./src/abs/domain.rs ./src/abs/semantics.rs ./src/abs/execution.rs

all: rdef_analysis ctrl_analysis gene_graphml

debug: rdef_analysis_debug ctrl_analysis_debug

test: test_rdef_analysis test_ctrl_analysis

rdef_analysis: $(RDEF_SRC)
	cargo build --release --bin rdef_analysis
	cp ./target/release/rdef_analysis ./rdef_analysis

ctrl_analysis: $(CTRL_SRC)
	cargo build --release --bin ctrl_analysis
	cp ./target/release/ctrl_analysis ./ctrl_analysis

rdef_analysis_debug: $(RDEF_SRC)
	cargo build --bin rdef_analysis
	cp ./target/debug/rdef_analysis ./rdef_analysis

ctrl_analysis_debug: $(CTRL_SRC)
	cargo build --bin ctrl_analysis
	cp ./target/debug/ctrl_analysis ./ctrl_analysis

gene_graphml: $(RDEF_SRC) $(CTRL_SRC)
	cargo build --release --bin gene_graphml
	cp ./target/release/gene_graphml ./gene_graphml

test_rdef_analysis: rdef_analysis
	@echo "Running rdef analysis on test files from ./demos/"
	@bash test-tests-rdef.sh
	@echo "Comparing the output with the expected output"
	@bash diff-tests-rdef.sh
	@echo ""

test_ctrl_analysis: ctrl_analysis
	@echo "Running ctrl analysis on test files from ./demos/"
	@bash test-tests-ctrl.sh
	@echo "Comparing the output with the expected output"
	@bash diff-tests-ctrl.sh
	@echo ""

package:
	mkdir -p assign-2
	cp -r src/ assign-2/src/
	cp ctrl.py assign-2/
	cp networkx-3.2.1-py3-none-any.whl assign-2/
	cp Cargo.toml Cargo.lock assign-2/
	cp Makefile assign-2/
	cp run-rdef.sh run-control.sh assign-2/
	cp build-analyses.sh assign-2/
	cd assign-2/ && zip -r ../assign-2.zip *

.PHONY: clean
clean:
	rm -f rdef_analysis ctrl_analysis gene_graphml *.tmp.*
	rm -rf src/__pycache__/
	rm -rf assign-2 assign-2.zip
	cargo clean
