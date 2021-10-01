merlin:
	cargo build --release

install:
	cargo install --path .

spellbook: spellbook.mn
	mkdir ~/.merlin
	cp spellbook.mn ~/.merlin
