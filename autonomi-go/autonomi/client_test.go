package autonomi

import (
	"testing"
)

func TestClientInitLocal(t *testing.T) {
	client, err := InitLocal()
	if err != nil {
		t.Fatalf("Failed to initialize local client: %v", err)
	}
	defer client.Close()

	if client.ptr == nil {
		t.Fatal("Client pointer is nil")
	}
}

func TestWalletCreation(t *testing.T) {
	// Test private key (from autonomi examples)
	privateKey := "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
	
	wallet, err := NewWalletFromPrivateKeyLocal(privateKey)
	if err != nil {
		t.Fatalf("Failed to create wallet: %v", err)
	}
	defer wallet.Close()

	if wallet.ptr == nil {
		t.Fatal("Wallet pointer is nil")
	}
}

func TestDataPutAndGet(t *testing.T) {
	// Skip if not in integration test mode
	if testing.Short() {
		t.Skip("Skipping integration test in short mode")
	}

	// Initialize client
	client, err := InitLocal()
	if err != nil {
		t.Fatalf("Failed to initialize client: %v", err)
	}
	defer client.Close()

	// Create wallet
	privateKey := "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
	wallet, err := NewWalletFromPrivateKeyLocal(privateKey)
	if err != nil {
		t.Fatalf("Failed to create wallet: %v", err)
	}
	defer wallet.Close()

	// Test data
	testData := []byte("Hello, Autonomi from Go!")

	// Upload data
	address, err := client.DataPut(testData, wallet)
	if err != nil {
		t.Fatalf("Failed to upload data: %v", err)
	}

	if address == "" {
		t.Fatal("Received empty address")
	}

	t.Logf("Data uploaded with address: %s", address)

	// Download data
	retrievedData, err := client.DataGet(address)
	if err != nil {
		t.Fatalf("Failed to retrieve data: %v", err)
	}

	// Verify data
	if string(retrievedData) != string(testData) {
		t.Fatalf("Data mismatch: expected %s, got %s", testData, retrievedData)
	}

	t.Log("Data successfully uploaded and retrieved!")
}