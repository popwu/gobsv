package libs

/*
#cgo CFLAGS: -I${SRCDIR}/lib
#cgo LDFLAGS: ${SRCDIR}/../lib/libbsv_go.a -ldl -lm
#include "../lib/main.h"
#include "../lib/private_key.h"
#include "../lib/public_key.h"
*/
import "C"
import (
	"errors"
	"unsafe"
)

type PublicKey struct {
	key *C.BSVPublicKey
}

// BSVPublicKey* publickey_from_hex(char* hex_str);
func PublicKeyFromHex(hexStr string) (*PublicKey, error) {
	key := C.publickey_from_hex(C.CString(string(hexStr)))
	if key == nil {
		return nil, errors.New("failed to create public key from hex")
	}
	return &PublicKey{key: key}, nil
}

// BSVPublicKey* publickey_from_bytes(unsigned char* bytes, size_t len);
func PublicKeyFromBytes(bytes []byte) (*PublicKey, error) {
	key := C.publickey_from_bytes((*C.uint8_t)(unsafe.Pointer(&bytes[0])), C.size_t(len(bytes)))
	if key == nil {
		return nil, errors.New("failed to create public key from bytes")
	}
	return &PublicKey{key: key}, nil
}

func (p *PublicKey) String() string {
	return p.ToHex()
}

// ByteArray publickey_to_bytes(BSVPublicKey* public_key);
func (p *PublicKey) ToBytes() []byte {
	b := C.publickey_to_bytes(p.key)
	return C.GoBytes(unsafe.Pointer(b.data), C.int(b.len))
}

// char* publickey_to_hex(BSVPublicKey* public_key);
func (p *PublicKey) ToHex() string {
	hex := C.publickey_to_hex(p.key)
	return C.GoString(hex)
}
