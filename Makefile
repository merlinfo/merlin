BIN = target/release/merlin
PBIN = /usr/bin/merlin

merlin:
	cargo build --release

install: $(BIN)
	cp $(BIN) $(PBIN)

uninstall: $(PBIN)
	rm /usr/bin/merlin

spellbook: spellbook.mn
	mkdir ~/.merlin
	cp spellbook.mn ~/.merlin
