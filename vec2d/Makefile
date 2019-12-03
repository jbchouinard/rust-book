SRC=$(shell find src -name *.rs)

release:
	cargo build --release

debug:
	cargo build

checkloop:
	find src -name *.rs | entr -s 'clear; cargo check'

.formatted: $(SRC)
	rustfmt $(SRC)
	touch $@

fmt: .formatted

run:
	cargo run

clean:
	rm -rf target

.PHONY: fmt clean release debug