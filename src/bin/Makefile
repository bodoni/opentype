build_dir := $(build_dir)/bin
binaries := $(notdir $(shell find . -maxdepth 1 ! -path . -type d))

all: $(binaries)

define define_binary
$(1)_src := $(shell find $(1) -name '*.rs')
$(1)_bin := $(build_dir)/$(1)

$(1): $$($(1)_bin)

$$($(1)_bin): $$($(1)_src) | $(build_dir)
	$(RUSTC) $(RUSTFLAGS) --out-dir $(build_dir) $(1)/main.rs

$(1)_clean:
	$(RM) $$($(1)_bin)

clean: $(1)_clean

.PHONY: $(1) $(1)_clean
endef

$(build_dir):
	$(MKDIR) $@

$(foreach binary,$(binaries),$(eval $(call define_binary,$(binary))))

$(benton_bin): $(shell find ../lib -name '*.rs')

clean:
	$(RM) -d $(build_dir)

.PHONY: all clean
