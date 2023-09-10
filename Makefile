.PHONY: clean test coverage lint

clean:
	@cargo clean
test:
	@cargo test
coverage: test
	@grcov target/coverage/grcov.profraw --branch --ignore-not-existing --binary-path target/debug/ -s . -t lcov --ignore "/*" -o target/coverage/lcov.info
lint:
	@cargo fix --allow-dirty --allow-staged
doc:
	@cargo +nightly doc --lib --no-deps --document-private-items
