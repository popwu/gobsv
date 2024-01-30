#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

typedef struct {
    unsigned char* data;
    size_t len;
} ByteArray;

typedef struct BSVPrivateKey BSVPrivateKey;
typedef struct BSVPublicKey BSVPublicKey;
typedef struct Signature Signature;
typedef struct ECIESCiphertext ECIESCiphertext;