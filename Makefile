TESTS := $(wildcard tests/*.sh)

build:
	cargo build --out-dir=./

test: build
	$(MAKE) $(TESTS)

$(TESTS):
	@echo 'Testing' $@
	@./$@
	@printf '\e[32mOK\e[0m\n'

clean:
	rm -rf out/



