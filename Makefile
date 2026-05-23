.PHONY: build test install clean

# Navescript build configuration
NVS_CONFIG = nvs.config.yaml

# Binaries
NVS_BIN = bin/nvs
NVM_BIN = bin/nvm

# Sources
GO_SRCS = $(shell find . -name "*.go")
ASM_SRCS = $(shell find internal/nasm_wasm -name "*.asm")
ASM_OBJS = $(ASM_SRCS:.asm=.o)

build: $(NVS_BIN) $(NVM_BIN) build-asm

$(NVS_BIN): $(GO_SRCS)
	go build -o $(NVS_BIN) ./cmd/nvs/main.go

$(NVM_BIN): $(GO_SRCS)
	go build -o $(NVM_BIN) ./cmd/nvm/main.go

# Assembly compilation (x86_64)
build-asm: $(ASM_OBJS)

%.o: %.asm
	nasm -f elf64 $< -o $@

test:
	go test ./internal/...

install: build
	cp $(NVS_BIN) /usr/local/bin/nvs
	cp $(NVM_BIN) /usr/local/bin/nvm

clean:
	rm -rf bin/
	rm -f $(ASM_OBJS)
