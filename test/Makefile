build_dir := $(build_dir)/test

all_src := $(shell find . -name '*.rs')
main_src := $(shell find . -name '*_test.rs')
module_src := $(filter-out $(main_src),$(all_src))
app_src := $(shell find $(source_dir) -name '*.rs')

tests := $(patsubst ./%.rs,%,$(main_src))
checks := $(patsubst %_test,%_check,$(tests))
executables := $(addprefix $(build_dir)/,$(tests))

environment := FIXTURE_PATH=$(test_dir)/fixtures

define define_test

$(1): $(build_dir)/$(1)

$(build_dir)/$(1): $(1).rs $(module_src) $(app_src)
	$(MKDIR) $(dir $(build_dir)/$(1))
	$(RUSTC) $(RUSTCFLAGS) --test -o $$@ $$<

$(patsubst %_test,%_check,$(1)): $(build_dir)/$(1)
	@$(environment) $$<

endef

$(foreach test,$(tests),$(eval $(call define_test,$(test))))

test: $(tests)

check: $(checks)

clean:
	$(RM) $(executables)

.PHONY: test $(tests) check $(checks) clean
