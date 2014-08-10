export make = $(MAKE)
export mkdir ?= mkdir -p
export rm ?= rm -f

export rustc ?= rustc
export rustflags ?= --opt-level=3

export base_dir := $(shell pwd)
export build_dir := $(base_dir)/build
export source_dir := $(base_dir)/src
export test_dir := $(base_dir)/test

all: $(build_dir)
	@$(make) -C $(source_dir) $@

check: $(build_dir)
	@$(make) -C $(test_dir) $@

$(build_dir):
	$(mkdir) $@

clean:
	@$(make) -C $(source_dir) clean
	@$(make) -C $(test_dir) clean

.PHONY: all check clean
