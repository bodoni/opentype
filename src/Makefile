export RUSTFLAGS := $(RUSTFLAGS) -L $(build_dir)/lib

all: bin

bin: lib
	@$(MAKE) -C bin all

lib:
	@$(MAKE) -C lib all

clean:
	@$(MAKE) -C bin clean
	@$(MAKE) -C lib clean

.PHONY: all bin lib clean
