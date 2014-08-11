build_dir := $(build_dir)/lib
libraries := $(notdir $(shell find . -type d -depth 1))

all: $(libraries)

define define_library
$(1)_src := $(shell find $(1) -name '*.rs')
$(1)_lib := $(build_dir)/lib$(1).rlib

$(1): $$($(1)_lib)

$$($(1)_lib): $$($(1)_src) | $(build_dir)
	$(RUSTC) $(RUSTFLAGS) --out-dir $(build_dir) $(1)/lib.rs

$(1)_clean:
	$(RM) $$($(1)_lib)

clean: $(1)_clean

.PHONY: $(1) $(1)_clean
endef

$(build_dir):
	$(MKDIR) $@

$(foreach library,$(libraries),$(eval $(call define_library,$(library))))

$(opentype_lib): $(input_lib)

clean:
	$(RM) -d $(build_dir)

.PHONY: all clean
