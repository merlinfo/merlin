merlin:
	cargo build --release
	strip target/release/merlin

install:
	cargo install --path .
	strip ~/.cargo/bin/merlin

spellbook: spellbook.mn
	mkdir ~/.merlin
	cp spellbook.mn ~/.merlin
