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

type PrivateKey struct {
	key *C.BSVPrivateKey
}

func PrivateKeyFromHex(hexStr string) (*PrivateKey, error) {
	key := C.privatekey_from_hex(C.CString(string(hexStr)))
	if key == nil {
		return nil, errors.New("failed to create private key from hex")
	}
	return &PrivateKey{key: key}, nil
}

func PrivateKeyFromRandom() (*PrivateKey, error) {
	key := C.privatekey_from_random()
	if key == nil {
		return nil, errors.New("failed to create private key from random")
	}
	return &PrivateKey{key: key}, nil
}

func PrivateKeyFromBytes(bytes []byte) (*PrivateKey, error) {
	key := C.privatekey_from_bytes((*C.uint8_t)(unsafe.Pointer(&bytes[0])), C.size_t(len(bytes)))
	if key == nil {
		return nil, errors.New("failed to create private key from bytes")
	}
	return &PrivateKey{key: key}, nil
}

func (p *PrivateKey) String() string {
	return p.ToHex()
}

func (p *PrivateKey) ToHex() string {
	hex := C.privatekey_to_hex(p.key)
	return C.GoString(hex)
}

func (p *PrivateKey) ToBytes() []byte {
	b := C.privatekey_to_bytes(p.key)
	return C.GoBytes(unsafe.Pointer(b.data), C.int(b.len))
}

func (p *PrivateKey) GetPoint() []byte {
	b := C.privatekey_get_point(p.key)
	return C.GoBytes(unsafe.Pointer(b.data), C.int(b.len))
}

// func (p *PrivateKey) CompressPublicKey(shouldCompress bool) (*PrivateKey, error) {
// 	var compress C.int
// 	if shouldCompress {
// 		compress = 1
// 	} else {
// 		compress = 0
// 	}
// 	key := C.privatekey_compress_public_key(p.key, compress)
// 	if key == nil {
// 		return nil, errors.New("failed to compress public key")
// 	}
// 	return &PrivateKey{key: key}, nil
// }

// func PrivateKeyFromWIF(wif string) (*PrivateKey, error) {
// 	key := C.privatekey_from_wif(C.CString(wif))
// 	if key == nil {
// 		return nil, errors.New("failed to create private key from WIF")
// 	}
// 	return &PrivateKey{key: key}, nil
// }

// func (p *PrivateKey) SignMessage(msg []byte) (*Signature, error) {
// 	signature := C.privatekey_sign_message(p.key, (*C.uint8_t)(unsafe.Pointer(&msg[0])), C.size_t(len(msg)))
// 	if signature == nil {
// 		return nil, errors.New("failed to sign message")
// 	}
// 	return &Signature{signature: signature}, nil
// }

// func (p *PrivateKey) ToWIF() string {
// 	wif := C.privatekey_to_wif(p.key)
// 	return C.GoString(wif)
// }

func (p *PrivateKey) ToPublicKey() (*PublicKey, error) {
	publicKey := C.privatekey_to_public_key(p.key)
	if publicKey == nil {
		return nil, errors.New("failed to get public key")
	}
	return &PublicKey{key: publicKey}, nil
}

// func (p *PrivateKey) EncryptMessage(message []byte) (*ECIESCiphertext, error) {
// 	ciphertext := C.privatekey_encrypt_message(p.key, (*C.uint8_t)(unsafe.Pointer(&message[0])), C.size_t(len(message)))
// 	if ciphertext == nil {
// 		return nil, errors.New("failed to encrypt message")
// 	}
// 	return &ECIESCiphertext{ciphertext: ciphertext}, nil
// }

// func (p *PrivateKey) DecryptMessage(ciphertext *ECIESCiphertext, senderPubKey *PublicKey) ([]byte, error) {
// 	decrypted := C.privatekey_decrypt_message(p.key, ciphertext.ciphertext, senderPubKey.key)
// 	if decrypted.data == nil {
// 		return nil, errors.New("failed to decrypt message")
// 	}
// 	return C.GoBytes(unsafe.Pointer(decrypted.data), C.int(decrypted.len)), nil
// }
