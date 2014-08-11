export RUSTFLAGS := $(RUSTFLAGS) -L $(build_dir)/libs

build_dir := $(build_dir)/test

export RUSTFLAGS := $(RUSTFLAGS) -L $(build_dir)

support_lib := $(build_dir)/libsupport.rlib

all: libs

libs: support
	@$(MAKE) -C libs all

support: $(support_lib)

$(support_lib): support.rs | $(build_dir)
	$(RUSTC) $(RUSTFLAGS) -o $@ $<

$(build_dir):
	$(MKDIR) $@

check: support
	@$(MAKE) -C libs check

clean:
	@$(MAKE) -C libs clean
	$(RM) $(support_lib)
	$(RM) -d $(build_dir)

.PHONY: all libs support check clean
