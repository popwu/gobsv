#ifndef TXOUT_H
#define TXOUT_H

#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

// Use this for user-defined BSVScript type
typedef void *BSVScript;

typedef struct {
  uint64_t value;
  // Opaque pointer to BSVScript
  BSVScript *script_pub_key;
} BSVTxOut;

// Function prototypes for TxOut

// Creates a new TxOut object
extern BSVTxOut *txout_new(uint64_t value, BSVScript *script_pub_key);

// Gets the satoshis value
extern uint64_t txout_get_satoshis(BSVTxOut *tx_out);

// Gets the satoshis value as bytes
extern ByteArray *txout_get_satoshis_as_bytes(BSVTxOut *tx_out);

// Gets the size of the script_pub_key
extern size_t txout_get_script_pub_key_size(BSVTxOut *tx_out);

// Gets the script_pub_key (user needs to handle memory management)
extern BSVScript *txout_get_script_pub_key(BSVTxOut *tx_out);

// Gets the script_pub_key as hex string
extern char *txout_get_script_pub_key_hex(BSVTxOut *tx_out);

// Creates a TxOut object from hex string
extern BSVTxOut *txout_from_hex(const char *hex_str);

// Converts the TxOut object to bytes
extern ByteArray *txout_to_bytes(BSVTxOut *tx_out);

// Converts the TxOut object to hex string
extern char *txout_to_hex(BSVTxOut *tx_out);

// Converts the TxOut object to JSON string
extern char *txout_to_json(BSVTxOut *tx_out);

// Converts the TxOut object to JSON string directly
extern char *txout_to_json_string(BSVTxOut *tx_out);

// User needs to free the memory allocated by these functions
extern void txout_free(BSVTxOut *tx_out);
extern void txout_free_bytes(ByteArray *bytes);
extern void txout_free_string(char *str);

#endif
