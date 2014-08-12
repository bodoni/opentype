export RUSTC ?= rustc
export RUSTFLAGS ?= --opt-level=3

export MKDIR ?= mkdir -p
export RM ?= rm -f

export base_dir := $(shell pwd)
export build_dir := $(base_dir)/build
export source_dir := $(base_dir)/src
export test_dir := $(base_dir)/test

all: bin

lib:
	@$(MAKE) -C $(source_dir) lib

bin:
	@$(MAKE) -C $(source_dir) bin

test: lib
	@$(MAKE) -C $(test_dir) all

check: lib
	@$(MAKE) -C $(test_dir) check

clean:
	@$(MAKE) -C $(source_dir) clean
	@$(MAKE) -C $(test_dir) clean
	$(RM) -d $(build_dir)

.PHONY: all lib bin test check clean
