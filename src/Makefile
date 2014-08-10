app := benton
executable := $(build_dir)/$(app)

all_src := $(shell find . -name '*.rs')
main_src := main.rs
module_src := $(filter-out $(main_src),$(all_src))

all: $(app)

$(app): $(executable)

$(executable): $(main_src) $(module_src)
	$(RUSTC) $(RUSTFLAGS) -o $@ $<

clean:
	$(RM) $(executable)

.PHONY: all $(app) clean
