.PHONY: clean

clean:
	cargo clean
test:
	cargo test
coverage: test
	grcov target/coverage/grcov.profraw --branch --ignore-not-existing --binary-path ./target/debug/ -s . -t lcov --ignore \"/*\" -o target/coverage/lcov.info