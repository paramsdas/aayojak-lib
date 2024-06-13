SHELL=/bin/bash
library:
	cd ./aayojak-lib/
	cargo build --lib
	cargo test
	cargo doc