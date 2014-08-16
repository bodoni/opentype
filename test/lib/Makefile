build_dir := $(build_dir)/lib
RUSTFLAGS := $(RUSTFLAGS) -L $(build_dir)

tests := $(patsubst ./%.rs,%,$(shell find . -name '*_test.rs'))
environment := FIXTURE_PATH=$(test_dir)/fixture
dependencies := $(shell find $(source_dir) -name '*.rs')

all: $(tests)

define define_test
$(1)_bin := $(build_dir)/$(1)
$(1)_dir := $(dir $(build_dir)/$(1))
$(1)_check := $(patsubst %_test,%_check,$(1))
$(1)_clean := $(patsubst %_test,%_clean,$(1))

$(1): $$($(1)_bin)

$$($(1)_bin): $(1).rs $(dependencies) | $$($(1)_dir)
	$(RUSTC) $(RUSTFLAGS) --test -o $$@ $$<

$$($(1)_check): $$($(1)_bin)
	@$(environment) $$<

check: $$($(1)_check)

$$($(1)_clean):
	$(RM) $$($(1)_bin)
	$(RM) -d $$($(1)_dir) 2> /dev/null || true

clean: $$($(1)_clean)

.PHONY: $(1) $$($(1)_check) $$($(1)_clean)
endef

define unique
	$(shell echo '$1' | xargs -n1 | sort -u | xargs)
endef

$(call unique,$(dir $(addprefix $(build_dir)/,$(tests)))):
	$(MKDIR) $@

$(foreach test,$(tests),$(eval $(call define_test,$(test))))

clean:
	$(RM) -d $(build_dir)

.PHONY: all check clean
