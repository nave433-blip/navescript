# NaveScript Installation Guide

## Automated Install
```bash
curl -fsSL https://get.navescript.org | bash
```

## Manual Build
1. Clone the repo: `git clone https://github.com/navescript/nvs.git`
2. Install dependencies: `make dev-deps`
3. Build the core: `make build`
4. Install to system: `sudo make install`
5. Verify: `nvs --version`

## Troubleshooting
- **Go Environment:** Ensure `go version` is 1.22+.
- **Permissions:** If installing globally, ensure `/usr/local/bin` is writable by your user.
- **Polyglot Engines:** Ensure `python3` and `node` are in your `PATH` for full polyglot support.
