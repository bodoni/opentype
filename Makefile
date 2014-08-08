VPATH = $(shell pwd)

RUSTC ?= rustc
RUSTFLAGS ?=

SOURCE_DIR = $(VPATH)/src
TARGET_DIR = $(VPATH)/build

TARGET = $(TARGET_DIR)/benton
SOURCES = $(shell find $(SOURCE_DIR) -type f -name '*.rs')

all: $(TARGET)

$(TARGET): $(SOURCES)
	$(RUSTC) $(RUSTFLAGS) $< -o $(TARGET)

clean:
	$(RM) $(TARGET) $(TARGET_DIR)/*.o
