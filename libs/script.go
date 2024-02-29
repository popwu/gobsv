package libs

/*
#cgo CFLAGS: -I${SRCDIR}/lib
#cgo LDFLAGS: ${SRCDIR}/../lib/libbsv_go.a -ldl -lm
#include "../lib/main.h"
#include "../lib/script.h"
*/
import "C"
import (
	"fmt"
	"unsafe"
)

type Script struct {
	key *C.BSVScript
}

func NewScriptFromASMString(asm string) (*Script, error) {
	var err C.Error
	key := C.script_from_asm_string(C.CString(asm), &err)
	if key == nil {
		return nil, fmt.Errorf("failed to create script from ASM string: %s", C.GoString(err.message))
	}
	return &Script{key: key}, nil
}

func NewScriptFromHex(hex string) (*Script, error) {
	var err C.Error
	key := C.script_from_hex(C.CString(hex), &err)
	if key == nil {
		return nil, fmt.Errorf("failed to create script from hex: %s", C.GoString(err.message))
	}
	return &Script{key: key}, nil
}

func NewScriptFromBytes(bytes []byte) (*Script, error) {
	var err C.Error
	key := C.script_from_bytes((*C.uint8_t)(&bytes[0]), C.size_t(len(bytes)), &err)
	if key == nil {
		return nil, fmt.Errorf("failed to create script from bytes: %s", C.GoString(err.message))
	}
	return &Script{key: key}, nil
}

func (p *Script) ToASMString() (string, error) {
	var err C.Error
	asm := C.script_to_asm_string(p.key, &err)
	if asm == nil {
		return "", fmt.Errorf("failed to convert script to ASM string: %s", C.GoString(err.message))
	}
	return C.GoString(asm), nil
}

func (p *Script) ToExtendedASMString() (string, error) {
	var err C.Error
	asm := C.script_to_extended_asm_string(p.key, &err)
	if asm == nil {
		return "", fmt.Errorf("failed to convert script to extended ASM string: %s", C.GoString(err.message))
	}
	return C.GoString(asm), nil
}

func (p *Script) ToScriptBits() (string, error) {
	var err C.Error
	bits := C.script_to_script_bits(p.key, &err)
	if bits == nil {
		return "", fmt.Errorf("failed to convert script to script bits: %s", C.GoString(err.message))
	}
	return C.GoString(bits), nil
}

func (p *Script) ToHex() (string, error) {
	var err C.Error
	hex := C.script_to_hex(p.key, &err)
	if hex == nil {
		return "", fmt.Errorf("failed to convert script to hex: %s", C.GoString(err.message))
	}
	return C.GoString(hex), nil
}

func (p *Script) ToBytes() []byte {
	bytes := C.script_to_bytes(p.key)
	return C.GoBytes(unsafe.Pointer(bytes.data), C.int(bytes.len))
}

func (p *Script) GetScriptLength() int {
	length := C.script_get_script_length(p.key)
	return int(length)
}

func (p *Script) RemoveCodeSeparators() {
	C.script_remove_codeseparators(p.key)
}

func EncodePushData(data []byte) ([]byte, error) {
	var err C.Error
	pushData := C.script_encode_pushdata((*C.uint8_t)(&data[0]), C.size_t(len(data)), &err)
	if pushData.data == nil {
		return nil, fmt.Errorf("failed to encode push data: %s", C.GoString(err.message))
	}
	return C.GoBytes(unsafe.Pointer(pushData.data), C.int(pushData.len)), nil
}

func GetPushDataBytes(length int) ([]byte, error) {
	var err C.Error
	pushData := C.script_get_pushdata_bytes(C.size_t(length), &err)
	if pushData.data == nil {
		return nil, fmt.Errorf("failed to get push data bytes: %s", C.GoString(err.message))
	}
	return C.GoBytes(unsafe.Pointer(pushData.data), C.int(pushData.len)), nil
}
