export RUSTC ?= rustc
export RUSTFLAGS ?= --opt-level=3

export MKDIR ?= mkdir -p
export RM ?= rm -f

export base_dir := $(shell pwd)
export build_dir := $(base_dir)/build
export source_dir := $(base_dir)/src
export test_dir := $(base_dir)/test

all: src

src:
	@$(MAKE) -C $(source_dir) all

test: src
	@$(MAKE) -C $(test_dir) all

check: src
	@$(MAKE) -C $(test_dir) check

clean:
	@$(MAKE) -C $(source_dir) clean
	@$(MAKE) -C $(test_dir) clean
	$(RM) -d $(build_dir)

.PHONY: all src test check clean
