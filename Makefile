CC = clang
SRC_DIR = cli
SRC_FILES = -I./$(SRC_DIR) $(SRC_DIR)/*.c
BUILD_DIR = target
LIBS = -L./$(BUILD_DIR)/debug -l:libvisel.a -lpthread -ldl

.PHONY: all library cli always

cli: library
	$(CC) -Wall -Wextra -o $(BUILD_DIR)/visel-cli $(SRC_FILES) $(LIBS)

library: headers
	cargo build

headers:
	cbindgen --config cbindgen.toml --crate visel --output bindings/visel.h --lang c

always:
	cargo install cbindgen
