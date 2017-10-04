# Verify using `cargo rustc -- --print=native-static-libs`
# MODE := release
LDFLAGS := -lutil -ldl -lrt -lpthread -lgcc_s -lc -lm -lrt -lpthread -lutil $(shell pkg-config gtk+-2.0 --libs)
TARGET_DIR := $(shell pwd)/target/debug
SHIMS := -u gtk_window_set_title

all:
	cargo build
	gcc -shared -D_GNU_SOURCE -fPIC -Isrc src/glue.c ${SHIMS} ${TARGET_DIR}/libgpatch.a ${LDFLAGS} -o ${TARGET_DIR}/libgtk-x11-2.0.so

.PHONY: all

