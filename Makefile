rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

rs_format:
	cargo fmt --quiet

rs_install:
	cargo install --path .

rs_lint:
	cargo clippy --quiet

rs_test:
	cargo test --quiet

rs_run:
	cargo run

rs_release:
	cargo build --release

rs_query:
	cargo 
