#ifndef BSV_TRANSACTION_H
#define BSV_TRANSACTION_H

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

typedef struct {
  void* data;
  size_t len;
} ByteArray;

typedef struct {
  ByteArray* data;
  size_t len;
} ByteArrayArray;

extern int transaction_get_version(BSVTransaction* transaction);
extern size_t transaction_get_ninputs(BSVTransaction* transaction);
extern size_t transaction_get_noutputs(BSVTransaction* transaction);
extern BSVTxIn* transaction_get_input(BSVTransaction* transaction, size_t index);
extern BSVTxOut* transaction_get_output(BSVTransaction* transaction, size_t index);
extern uint32_t transaction_get_n_locktime(BSVTransaction* transaction);
extern ByteArray transaction_get_n_locktime_as_bytes(BSVTransaction* transaction);
extern BSVTransaction* transaction_new(uint32_t version, uint32_t n_locktime);
extern BSVTransaction* transaction_set_version(BSVTransaction* transaction, uint32_t version);
extern BSVTransaction* transaction_set_nlocktime(BSVTransaction* transaction, uint32_t n_locktime);
extern void transaction_add_input(BSVTransaction* transaction, BSVTxIn* input);
extern void transaction_prepend_input(BSVTransaction* transaction, BSVTxIn* input);
extern void transaction_insert_input(BSVTransaction* transaction, size_t index, BSVTxIn* input);
extern void transaction_add_output(BSVTransaction* transaction, BSVTxOut* output);
extern void transaction_prepend_output(BSVTransaction* transaction, BSVTxOut* output);
extern void transaction_insert_output(BSVTransaction* transaction, size_t index, BSVTxOut* output);
extern void transaction_set_input(BSVTransaction* transaction, size_t index, BSVTxIn* input);
extern void transaction_set_output(BSVTransaction* transaction, size_t index, BSVTxOut* output);
extern bool transaction_is_coinbase_impl(BSVTransaction* transaction);
extern int64_t transaction_satoshis_in(BSVTransaction* transaction);
extern uint64_t transaction_satoshis_out(BSVTransaction* transaction);
extern BSVTransaction* transaction_from_hex(const char* hex_str);
extern BSVTransaction* transaction_from_bytes(const ByteArray* tx_bytes);
extern char* transaction_to_json_string(BSVTransaction* transaction);
extern BSVTransaction* transaction_from_json_string(const char* json_string);
extern char* transaction_to_json(BSVTransaction* transaction);
extern ByteArray* transaction_to_bytes(BSVTransaction* transaction);
extern char* transaction_to_hex(BSVTransaction* transaction);
extern size_t transaction_get_size(BSVTransaction* transaction);
extern void transaction_add_inputs(BSVTransaction* transaction, BSVTxIn* tx_ins, size_t len);
extern ByteArrayArray transaction_get_outpoints(BSVTransaction* transaction);
extern void transaction_add_outputs(BSVTransaction* transaction, BSVTxOut** tx_outs, size_t len);
extern char* transaction_get_id_hex(BSVTransaction* transaction);
extern ByteArray* transaction_get_id_bytes(BSVTransaction* transaction);
extern ByteArray* transaction_to_compact_bytes(BSVTransaction* transaction);
extern char* transaction_to_compact_hex(BSVTransaction* transaction);
extern BSVTransaction* transaction_from_compact_bytes(const u8* data, size_t len);
extern BSVTransaction* transaction_from_compact_hex(const char* compact_hex);
extern bool transaction_is_coinbase(BSVTransaction* transaction);
extern SighashSignature* transaction_sign(
    BSVTransaction* transaction,
    PrivateKey* priv_key,
    int32_t sighash,
    size_t n_tx_in,
    Script* unsigned_script,
    uint64_t value);

extern SighashSignature* transaction_sign_with_k(
    BSVTransaction* transaction,
    PrivateKey* priv_key,
    PrivateKey* ephemeral_key,
    int sighash,
    size_t n_tx_in,
    Script* unsigned_script,
    uint64_t value
);

extern ByteArray* transaction_sighash_preimage(
    BSVTransaction* transaction,
    int sighash,
    size_t n_tx_in,
    Script* unsigned_script,
    uint64_t value
);

extern bool transaction_verify(
    BSVTransaction* transaction,
    PublicKey* pub_key,
    SighashSignature* sig
);

extern bool transaction__verify(
    BSVTransaction* transaction,
    PublicKey* pub_key,
    SighashSignature* sig,
    bool reverse_digest
);
    
#endif // BSV_TRANSACTION_H