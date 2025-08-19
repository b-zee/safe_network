package main

import (
	"fmt"
	"log"
	"os"

	autonomi "github.com/autonomi/autonomi-go/autonomi"
)

func main() {
	// Check for required environment variable
	privateKey := os.Getenv("SECRET_KEY")
	if privateKey == "" {
		log.Fatal("Please set SECRET_KEY environment variable with your EVM private key")
	}

	// Initialize client connected to local network
	fmt.Println("Connecting to local Autonomi network...")
	client, err := autonomi.InitLocal()
	if err != nil {
		log.Fatalf("Failed to initialize client: %v", err)
	}
	defer client.Close()
	fmt.Println("✓ Connected successfully")

	// Create wallet from private key
	fmt.Println("Creating wallet...")
	wallet, err := autonomi.NewWalletFromPrivateKeyLocal(privateKey)
	if err != nil {
		log.Fatalf("Failed to create wallet: %v", err)
	}
	defer wallet.Close()
	fmt.Println("✓ Wallet created")

	// Upload data
	data := []byte("Hello from Go bindings!")
	fmt.Printf("Uploading data: %s\n", string(data))
	
	address, err := client.DataPut(data, wallet)
	if err != nil {
		log.Fatalf("Failed to upload data: %v", err)
	}
	fmt.Printf("✓ Data uploaded with address: %s\n", address)

	// Retrieve data
	fmt.Println("Retrieving data...")
	retrievedData, err := client.DataGet(address)
	if err != nil {
		log.Fatalf("Failed to retrieve data: %v", err)
	}
	fmt.Printf("✓ Retrieved data: %s\n", string(retrievedData))

	// Verify data integrity
	if string(retrievedData) == string(data) {
		fmt.Println("✓ Data integrity verified!")
	} else {
		fmt.Println("✗ Data mismatch!")
	}
}