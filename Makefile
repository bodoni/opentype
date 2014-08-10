export MKDIR ?= mkdir -p
export RM ?= rm -f

export RUSTC ?= rustc
export RUSTFLAGS ?= --opt-level=3

export base_dir := $(shell pwd)
export build_dir := $(base_dir)/build
export source_dir := $(base_dir)/src
export test_dir := $(base_dir)/test

all: $(build_dir)
	@$(MAKE) -C $(source_dir) $@

check: $(build_dir)
	@$(MAKE) -C $(test_dir) $@

$(build_dir):
	$(MKDIR) $@

clean:
	@$(MAKE) -C $(source_dir) clean
	@$(MAKE) -C $(test_dir) clean

.PHONY: all check clean
