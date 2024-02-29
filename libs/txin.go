package libs

/*
#cgo CFLAGS: -I${SRCDIR}/lib
#cgo LDFLAGS: ${SRCDIR}/../lib/libbsv_go.a -ldl -lm
#include "../lib/main.h"
#include "../lib/txin.h"
*/
import "C"
import (
	"errors"
	"unsafe"
)

type TxIn struct {
	key *C.BSVTxIn
}

// BSVPublicKey* publickey_from_hex(char* hex_str);
func NewTxIn(prevTxId []byte, vout uint32, unlockingScript *Script, sequence uint32) *TxIn {
	key := C.txin_new((*C.uint8_t)(unsafe.Pointer(&prevTxId[0])), C.size_t(len(prevTxId)), C.uint32_t(vout), unlockingScript.key, (*C.uint32_t)(unsafe.Pointer(&sequence)))
	return &TxIn{key: key}
}

// BSVPublicKey* publickey_from_bytes(unsigned char* bytes, size_t len);
func PublicKeyFromBytes(bytes []byte) (*TxIn, error) {
	key := C.publickey_from_bytes((*C.uint8_t)(unsafe.Pointer(&bytes[0])), C.size_t(len(bytes)))
	if key == nil {
		return nil, errors.New("failed to create public key from bytes")
	}
	return &PublicKey{key: key}, nil
}

func (p *TxIn) GetVout() string {
	return C.txin_get_vout(p.key)
}

// ByteArray publickey_to_bytes(BSVPublicKey* public_key);
// func (p *TxIn) ToBytes() []byte {
// 	b := C.publickey_to_bytes(p.key)
// 	return C.GoBytes(unsafe.Pointer(b.data), C.int(b.len))
// }

// // char* publickey_to_hex(BSVPublicKey* public_key);
// func (p *TxIn) ToHex() string {
// 	hex := C.publickey_to_hex(p.key)
// 	return C.GoString(hex)
// }
