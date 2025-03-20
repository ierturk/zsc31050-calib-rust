## Copyright (c) 2025, Ibrahim Erturk <me@erturk.me>, ErturkMe
## Licensed under the BSD 3-Clause License.
## See the LICENSE file in the project root for more information.

# Define the targets
WINDOWS_x86_TARGET = i686-pc-windows-msvc
WINDOWS_x86_64_TARGET = x86_64-pc-windows-msvc

default: all

# Define the build commands
server:
	cargo build --target $(WINDOWS_x86_TARGET) --no-default-features -p rbic1_dll_svc --example server

client:
	cargo build --target $(WINDOWS_x86_64_TARGET) --no-default-features -p rbic1_dll_svc --example client

gui:
	cargo build --target $(WINDOWS_x86_64_TARGET) -p test_gui

build-all: server client gui

# Define the clean commands
clean-rbic1-svc:
	cargo clean -p rbic1_dll_svc

clean-gui:
	cargo clean -p test_gui

clean:
	cargo clean

clean-all: clean-server clean-client clean-gui

# Define the run commands
run-server:
	cargo run --target $(WINDOWS_x86_TARGET) --no-default-features -p rbic1_dll_svc --example server

run-client:
	cargo run --target $(WINDOWS_x86_64_TARGET) --no-default-features -p rbic1_dll_svc --example client

run-gui:
	cargo run --target $(WINDOWS_x86_64_TARGET) -p test_gui --bin test_gui

format:
	cargo fmt

# Define the default target
.PHONY: all
all: build-all
