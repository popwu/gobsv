// #include <stdint.h>
// #include <stddef.h>

// typedef struct {
//     uint8_t* data;
//     size_t len;
// } ByteArray;

// typedef struct BSVScript BSVScript;

char* script_to_asm_string(BSVScript* script, Error* error);
char* script_to_extended_asm_string(BSVScript* script, Error* error);
BSVScript* script_from_hex(char* hex, Error* error);
BSVScript* script_from_bytes(uint8_t* bytes, size_t len, Error* error);
BSVScript* script_from_asm_string(char* asm_string, Error* error);
ByteArray script_encode_pushdata(uint8_t* data_bytes, size_t len, Error* error);
ByteArray script_get_pushdata_bytes(size_t length, Error* error);
char* script_to_script_bits(BSVScript* script, Error* error);
ByteArray script_to_bytes(BSVScript* script);
size_t script_get_script_length(BSVScript* script);
char* script_to_hex(BSVScript* script, Error* error);
void script_remove_codeseparators(BSVScript* script);