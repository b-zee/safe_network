# Autonomi Go Bindings

Go bindings for the Autonomi Network client library.

## Overview

These bindings provide a Go interface to the Autonomi Network, allowing Go developers to:
- Connect to the Autonomi Network (local or mainnet)
- Upload and retrieve data
- Manage EVM wallets for payments
- Interact with the decentralized storage network

## Architecture

The bindings use CGO to interface with a Rust FFI library (`autonomi-ffi`). The architecture consists of:

1. **Rust FFI Layer** (`ffi/`): Exposes C-compatible functions from the Autonomi Rust client
2. **C Header** (`ffi/autonomi.h`): Auto-generated header file for CGO
3. **Go Wrapper** (`autonomi/`): Idiomatic Go API that wraps the FFI calls

## Building

### Prerequisites

- Rust toolchain (1.75+)
- Go (1.21+)
- C compiler (gcc/clang)

### Build Steps

```bash
# Build the FFI library and run tests
./build.sh
```

## Usage

### Basic Example

```go
package main

import (
    "fmt"
    "log"
    autonomi "github.com/autonomi/autonomi-go/autonomi"
)

func main() {
    // Connect to local network
    client, err := autonomi.InitLocal()
    if err != nil {
        log.Fatal(err)
    }
    defer client.Close()

    // Create wallet from private key
    wallet, err := autonomi.NewWalletFromPrivateKeyLocal("0x...")
    if err != nil {
        log.Fatal(err)
    }
    defer wallet.Close()

    // Upload data
    data := []byte("Hello, Autonomi!")
    address, err := client.DataPut(data, wallet)
    if err != nil {
        log.Fatal(err)
    }
    fmt.Printf("Data uploaded: %s\n", address)

    // Retrieve data
    retrieved, err := client.DataGet(address)
    if err != nil {
        log.Fatal(err)
    }
    fmt.Printf("Data retrieved: %s\n", string(retrieved))
}
```

## Testing

Run the test suite:

```bash
cd autonomi
go test -v
```

For integration tests (requires local network):

```bash
# Start local network first
cargo run --bin antctl -- local run --build --clean

# Run integration tests
go test -v
```

## API Reference

### Client

- `Init() (*Client, error)` - Connect to mainnet
- `InitLocal() (*Client, error)` - Connect to local network
- `DataPut(data []byte, wallet *Wallet) (string, error)` - Upload data
- `DataGet(address string) ([]byte, error)` - Retrieve data
- `Close()` - Release resources

### Wallet

- `NewWalletFromPrivateKeyLocal(privateKey string) (*Wallet, error)` - Create wallet for local network
- `Close()` - Release resources

## Memory Management

The bindings use Go's `runtime.SetFinalizer` to automatically clean up resources when objects are garbage collected. However, it's recommended to explicitly call `Close()` on clients and wallets when done to ensure timely resource cleanup.

## Error Handling

Errors from the FFI layer are wrapped in Go error types with error codes and messages preserved:

```go
type Error struct {
    Code    ErrorCode
    Message string
}
```

Error codes include:
- `InvalidArgument` - Invalid input parameters
- `ConnectionError` - Network connection issues
- `RuntimeError` - Runtime/execution errors
- `NullPointer` - Null pointer access
- `InvalidUtf8` - UTF-8 encoding issues

## Extending the Bindings

To add new functionality:

1. Add the Rust function to `ffi/src/lib.rs`
2. Rebuild to regenerate the C header
3. Add the Go wrapper in `autonomi/client.go`
4. Add tests in `autonomi/client_test.go`

## License

This project is licensed under the GPL-3.0 License - see the parent repository for details.