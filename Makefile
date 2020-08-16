int:
	cargo test --test lib --no-fail-fast --features "json"

unit:
	cargo test --locked  --no-fail-fast --lib

test: unit int