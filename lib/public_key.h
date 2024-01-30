

char* publickey_to_address(BSVPublicKey* public_key);
BSVPublicKey* publickey_from_hex(char* hex_str);
BSVPublicKey* publickey_from_bytes(unsigned char* bytes, size_t len);
ByteArray publickey_to_bytes(BSVPublicKey* public_key);
char* publickey_to_hex(BSVPublicKey* public_key);
BSVPublicKey* publickey_from_private_key(BSVPrivateKey* private_key);
bool publickey_verify_message(BSVPublicKey* public_key, const unsigned char* message, size_t message_len, Signature* signature);
char* publickey_to_p2pkh_address(BSVPublicKey* public_key);
BSVPublicKey* publickey_to_compressed(BSVPublicKey* public_key);
BSVPublicKey* publickey_to_decompressed(BSVPublicKey* public_key);
ECIESCiphertext* publickey_encrypt_message(BSVPublicKey* public_key, const unsigned char* message, size_t message_len, BSVPrivateKey* sender_private_key);
bool publickey_is_valid_message(BSVPublicKey* public_key, const unsigned char* message, size_t message_len, Signature* signature);
bool publickey_is_compressed(BSVPublicKey* public_key);