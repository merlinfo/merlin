merlin:
	cargo build --release

install: $(BIN)
	cargo install --path .

spellbook: spellbook.mn
	mkdir ~/.merlin
	cp spellbook.mn ~/.merlin
