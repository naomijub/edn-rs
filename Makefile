integration:
	cargo test --test lib --no-fail-fast --features "json"

unit:
	cargo test  --no-fail-fast --lib --features "json"

.PHONY: examples
examples:
	cargo test --examples --no-fail-fast
	cargo test --example json_to_edn --features "json"
	cargo run --example async --features "async"

doc-tests:
	cargo test --doc

test: unit integration examples doc-tests
