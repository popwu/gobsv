#ifndef TXIN_H
#define TXIN_H

#include <stdint.h>
#include <stdbool.h>

typedef struct {
  uint8_t *data;
  size_t len;
} ByteArray;

// Function prototypes for TxIn

// Creates a new TxIn object
extern BSVTxIn *txin_new(
    uint8_t *prev_tx_id, size_t prev_tx_id_len,
    uint32_t vout, BSVScript *unlocking_script,
    uint32_t *sequence);

// Creates an empty TxIn object
extern BSVTxIn *txin_empty();

// Gets the previous transaction ID
extern ByteArray *txin_get_prev_tx_id(BSVTxIn *txin, bool little_endian);

// Gets the previous transaction ID as hex string
extern char *txin_get_prev_tx_id_hex(BSVTxIn *txin, bool little_endian);

// Gets the output index (vout)
extern uint32_t txin_get_vout(BSVTxIn *txin);

// Gets the size of the unlocking script
extern uint64_t txin_get_unlocking_script_size(BSVTxIn *txin);

// Gets the unlocking script
extern BSVScript *txin_get_unlocking_script(BSVTxIn *txin);

// Gets the unlocking script as hex string
extern char *txin_get_unlocking_script_hex(BSVTxIn *txin);

// Gets the sequence number
extern uint32_t txin_get_sequence(BSVTxIn *txin);

// Gets the sequence number as bytes
extern ByteArray *txin_get_sequence_as_bytes(BSVTxIn *txin);

// Gets the outpoint as bytes
extern ByteArray *txin_get_outpoint_bytes(BSVTxIn *txin, bool little_endian);

// Gets the outpoint as hex string
extern char *txin_get_outpoint_hex(BSVTxIn *txin, bool little_endian);

// Sets the unlocking script
extern void txin_set_unlocking_script(BSVTxIn *txin, BSVScript *script);

// Sets the previous transaction ID
extern void txin_set_prev_tx_id(BSVTxIn *txin, uint8_t *txid, size_t len);

// Sets the output index (vout)
extern void txin_set_vout(BSVTxIn *txin, uint32_t vout);

// Sets the sequence number
extern void txin_set_sequence(BSVTxIn *txin, uint32_t sequence);

// Sets the satoshis (deprecated)
extern void txin_set_satoshis(BSVTxIn *txin, uint64_t satoshis);

// Gets the satoshis (deprecated)
extern uint64_t txin_get_satoshis(BSVTxIn *txin);

// Sets the locking script (deprecated)
extern void txin_set_locking_script(BSVTxIn *txin, BSVScript *locking_script);

// Gets the locking script (deprecated)
extern BSVScript *txin_get_locking_script(BSVTxIn *txin);

// Gets the locking script as bytes (deprecated)
extern ByteArray *txin_get_locking_script_bytes(BSVTxIn *txin);

// Creates a TxIn object from a hex string
extern BSVTxIn *txin_from_hex(char *hex_str);

// Converts the TxIn object to JSON string
extern char *txin_to_json(BSVTxIn *txin);

// Converts the TxIn object to JSON string directly
extern char *txin_to_json_string(BSVTxIn *txin);

// Converts the TxIn object to bytes
extern ByteArray *txin_to_bytes(BSVTxIn *txin);

// Converts the TxIn object to hex string
extern char *txin_to_hex(BSVTxIn *txin);

// Creates a TxIn object from outpoint bytes
extern BSVTxIn *txin_from_outpoint_bytes(uint8_t *outpoint, size_t len);

// Converts the TxIn object
#endif // TXIN_H