export RUSTFLAGS := $(RUSTFLAGS) -L $(build_dir)/lib

build_dir := $(build_dir)/test

export RUSTFLAGS := $(RUSTFLAGS) -L $(build_dir)

support_lib := $(build_dir)/libsupport.rlib

all: lib

lib: support
	@$(MAKE) -C lib all

support: $(support_lib)

$(support_lib): support.rs | $(build_dir)
	$(RUSTC) $(RUSTFLAGS) -o $@ $<

$(build_dir):
	$(MKDIR) $@

check: support
	@$(MAKE) -C lib check

clean:
	@$(MAKE) -C lib clean
	$(RM) $(support_lib)
	$(RM) -d $(build_dir)

.PHONY: all lib support check clean
