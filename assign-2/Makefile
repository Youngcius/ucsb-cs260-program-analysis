RDEF_SRC = ./src/bin/rdef.rs ./src/cfg.rs ./src/lir.rs ./src/store.rs ./src/utils.rs ./src/abs/domain.rs ./src/abs/semantics.rs ./src/abs/execution.rs
CTRL_SRC = ./src/bin/ctrl.rs ./src/cfg.rs ./src/lir.rs ./src/store.rs ./src/utils.rs ./src/abs/domain.rs ./src/abs/semantics.rs ./src/abs/execution.rs

all: rdef_analysis ctrl_analysis

debug: rdef_analysis_debug ctrl_analysis_debug

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

package:
	mkdir -p assign-2
	cp -r src/ assign-2/src/
	cp Cargo.toml Cargo.lock assign-2/
	cp Makefile assign-2/
	cp run-rdef.sh run-control.sh assign-2/
	cp build-analyses.sh assign-2/
	cd assign-2/ && zip -r ../assign-2.zip *

.PHONY: clean
clean:
	rm -f rdef_analysis ctrl_analysis *.out.tmp.*
	rm -rf assign-2 assign-2.zip
	cargo clean
