package gobsv

/*
#cgo LDFLAGS: ${SRCDIR}/lib/libbsv_go.a -ldl -lm
#include "lib/main.h"
#include "lib/private_key.h"
#include "lib/public_key.h"
*/
import "C"
import (
	"errors"
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

func (p *PrivateKey) String() string {
	return p.ToHex()
}

func (p *PrivateKey) ToHex() string {
	hex := C.privatekey_to_hex(p.key)
	return C.GoString(hex)
}

// // NewPrivateKey 创建一个新的 PrivateKey 实例
// func NewPrivateKey() *PrivateKey {
// 	return &PrivateKey{
// 		key: C.bsv_private_key_new(),
// 	}
// }

// // Free 释放私钥的内存
// func (p *PrivateKey) Free() {
// 	C.bsv_private_key_free(p.key)
// }

// // ToWIF 将私钥转换为 WIF 格式
// func (p *PrivateKey) ToWIF() string {
// 	wif := C.bsv_private_key_to_wif(p.key)
// 	return C.GoString(wif)
// }

// // FromWIF 从 WIF 格式的字符串创建私钥
// func (p *PrivateKey) FromWIF(wif string) error {
// 	wifC := C.CString(wif)
// 	defer C.free(unsafe.Pointer(wifC))
// 	result := C.bsv_private_key_from_wif(p.key, wifC)
// 	if result != 0 {
// 		return errors.New("failed to create private key from WIF")
// 	}
// 	return nil
// }
