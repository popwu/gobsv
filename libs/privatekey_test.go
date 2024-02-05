package libs

import (
	"fmt"
	"testing"
	// "github.com/popwu/gobsv/libs"
)

func TestPrivateKey(t *testing.T) {
	hexStr := "c473061e73df28bd13cbaa5deeb0c6a6ed83ed6d449048905741586981c3ec50"
	p, err := PrivateKeyFromHex(hexStr)
	if err != nil {
		t.Fatalf("PrivateKeyFromHex failed with error: %v", err)
	}
	fmt.Printf("p: %T %s\n", p, p)
	fmt.Printf("p.key: %s\n", p.ToHex())

	if p.ToHex() != hexStr {
		t.Errorf("Expected hex string %s, but got %s", hexStr, p.ToHex())
	}

	pub, err := p.ToPublicKey()
	fmt.Printf("pub: %T %s\n", pub, pub)
}
