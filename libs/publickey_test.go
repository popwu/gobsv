package libs

import (
	"fmt"
	"testing"
	// "github.com/popwu/gobsv/libs"
)

func TestPublicKey(t *testing.T) {
	hexStr := "035a20336e5bce37132548aed9201a6f82aa27067af2f43afdcfa2454fd90b4fd5"
	p, err := PublicKeyFromHex(hexStr)
	if err != nil {
		t.Fatalf("PrivateKeyFromHex failed with error: %v", err)
	}
	fmt.Printf("p: %T %s\n", p, p)
	fmt.Printf("p.key: %s\n", p.ToHex())

	if p.ToHex() != hexStr {
		t.Errorf("Expected hex string %s, but got %s", hexStr, p.ToHex())
	}
}
