RUSTC ?= rustc
RUSTFLAGS ?=

SOURCE_DIR = src
TARGET_DIR = build

TARGET = $(TARGET_DIR)/benton
SOURCES = $(shell find $(SOURCE_DIR) -type f -name '*.rs')

all: $(TARGET)

$(TARGET): $(SOURCES)
	$(RUSTC) $(RUSTFLAGS) $< -o $(TARGET)

clean:
	$(RM) $(TARGET) $(TARGET_DIR)/*.o
