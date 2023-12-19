integration:
	cargo test --test lib --no-fail-fast --features "json"

unit:
	cargo test  --no-fail-fast --lib --features "json"

.PHONY: examples
examples:
	cargo test --examples --no-fail-fast
	cargo test --example json_to_edn --features "json"
	cargo test --example edn_to_json --features "json"
	cargo run --example async
	cargo run --example struct_from_str --no-default-features

tests:
	cargo test --all-features --no-fail-fast
	cargo test --no-default-features --no-fail-fast
	cargo test --doc

clippy:
	cargo clippy --all-features -- -W future-incompatible -W rust_2018_idioms -W clippy::all -W clippy::pedantic -W clippy::nursery --deny warnings
	cargo clippy --features json --no-default-features -- -W future-incompatible -W rust_2018_idioms -W clippy::all -W clippy::pedantic -W clippy::nursery --deny warnings
	cargo clippy --no-default-features -- -W future-incompatible -W rust_2018_idioms -W clippy::all -W clippy::pedantic -W clippy::nursery --deny warnings

fmt:
	cargo fmt --check

test: unit integration examples tests clippy fmt
