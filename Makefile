.PHONY: clean coverage

clean:
	@cargo clean; \
	rm *.profraw
test:
	@cargo test
coverage: test
	@grcov target/coverage/grcov.profraw --branch --ignore-not-existing --binary-path ./target/debug/ -s . -t lcov --ignore \"/*\" -o target/coverage/lcov.info
lint:
	@cargo fix --allow-dirty --allow-staged
