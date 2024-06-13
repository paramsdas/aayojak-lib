library:
	cargo build --lib --manifest-path=./aayojak-lib/Cargo.toml
	cargo test --manifest-path=./aayojak-lib/Cargo.toml
	cargo doc --manifest-path=./aayojak-lib/Cargo.toml
