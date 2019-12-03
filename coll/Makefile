SRC=$(shell find src -name *.rs)

all: fmt release debug

fmt:
	rustfmt $(SRC)

debug:
	cargo build

release:
	cargo build --release

checkloop:
	find src -name *.rs | entr -s 'clear; cargo check'

run:
	cargo run

clean:
	rm -rf target

.PHONY: all fmt debug release checkloop run clean