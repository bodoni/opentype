BASE_DIR   ?= $(shell pwd)
BUILD_DIR  ?= $(BASE_DIR)/build

RUSTC      ?= rustc
RUSTCFLAGS := --opt-level=3

PROGRAM    := benton

SOURCE_DIR := $(BASE_DIR)/src
MAIN_SRC   := $(SOURCE_DIR)/main.rs
MODULE_SRC := $(shell find $(SOURCE_DIR) \( -name *.rs ! -name main.rs \))

TEST_DIR   := $(BASE_DIR)/test
TESTS      := $(shell find $(TEST_DIR) -name *.rs)
TESTS      := $(patsubst $(TEST_DIR)/%.rs,%,$(TESTS))

all: $(PROGRAM)

$(PROGRAM): $(BUILD_DIR)/$(PROGRAM)

$(BUILD_DIR)/$(PROGRAM): $(MAIN_SRC) $(MODULE_SRC) | $(BUILD_DIR)
	$(RUSTC) $(RUSTCFLAGS) -o $@ $<

$(BUILD_DIR):
	mkdir $@

define TEST
test_$(1): $(BUILD_DIR)/test_$(1)

$(BUILD_DIR)/test_$(1): $(TEST_DIR)/$(1).rs | $(BUILD_DIR)
	$(RUSTC) $(RUSTCFLAGS) --test -o $$@ $$^

check_$(1): test_$(1)
	$(BUILD_DIR)/test_$(1)
endef

$(foreach test,$(TESTS),$(eval $(call TEST,$(test))))

test: $(addprefix test_,$(TESTS))

check: $(addprefix check_,$(TESTS))

clean:
	rm -rf "$(BUILD_DIR)"

.PHONY: all $(PROGRAM) $(addprefix test_,$(TESTS))\
	$(addprefix check_,$(TESTS)) check test clean
