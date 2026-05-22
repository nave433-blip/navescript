.PHONY: build test install clean

build:
	go build -o bin/nvs ./cmd/nvs/main.go
	go build -o bin/nvm ./cmd/nvm/main.go

test:
	go test ./internal/...

install:
	cp bin/nvs /usr/local/bin/nvs
	cp bin/nvm /usr/local/bin/nvm

clean:
	rm -rf bin/
