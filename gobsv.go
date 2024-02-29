package main

import (
	"fmt"
	"gobsv/libs"
)

func main() {
	pub, err := libs.PublicKeyFromHex("test")
	fmt.Println(pub, err)

	txin := libs.NewTxIn([]byte("test"), 1, nil, 1)
	fmt.Println(txin)
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
