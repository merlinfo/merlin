merlin:
	cargo build --release
	strip target/release/merlin

install:
	cargo install --path .

spellbook: spellbook.mn
	mkdir ~/.merlin
	cp spellbook.mn ~/.merlin
