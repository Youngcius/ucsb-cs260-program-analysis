RDEF_SRC = ./src/bin/rdef.rs ./src/abs.rs ./src/cfg.rs ./src/lir.rs ./src/store.rs ./src/utils.rs
CTRL_SRC = ./src/bin/ctrl.rs ./src/abs.rs ./src/cfg.rs ./src/lir.rs ./src/store.rs ./src/utils.rs

all: reach_definition_analysis control_dependence_analysis

debug: reach_definition_analysis_debug control_dependence_analysis_debug

reach_definition_analysis: $(RDEF_SRC)
	cargo build --release --bin reach_definition_analysis
	cp ./target/release/reach_definition_analysis ./reach_definition_analysis

control_dependence_analysis: $(CTRL_SRC)
	cargo build --release --bin control_dependence_analysis
	cp ./target/release/control_dependence_analysis ./control_dependence_analysis

reach_definition_analysis_debug: $(RDEF_SRC)
	cargo build --bin reach_definition_analysis
	cp ./target/debug/reach_definition_analysis ./reach_definition_analysis

control_dependence_analysis_debug: $(CTRL_SRC)
	cargo build --bin control_dependence_analysis
	cp ./target/debug/control_dependence_analysis ./control_dependence_analysis

package:
	mkdir -p assign-2
	cp -r src/ assign-2/
	cp Cargo.toml Cargo.lock assign-2/
	cp Makefile assign-2/
	cp run-rdef.sh run-control.sh assign-2/
	cp build-analyses.sh assign-2/
	cd assign-2/ && zip -r ../assign-2.zip *

.PHONY: clean
clean:
	rm -f reach_definition_analysis control_dependence_analysis
	rm -rf assign-2 assign-2.zip
	cargo clean
