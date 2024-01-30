


// privatekey.to_bytes
ByteArray privatekey_to_bytes(struct BSVPrivateKey* private_key);

// privatekey.to_hex
char* privatekey_to_hex(struct BSVPrivateKey* private_key);

// privatekey.from_random
struct BSVPrivateKey* privatekey_from_random();

// privatekey.get_point
ByteArray privatekey_get_point(struct BSVPrivateKey* private_key);

// privatekey.compress_public_key
struct BSVPrivateKey* privatekey_compress_public_key(struct BSVPrivateKey* private_key, int should_compress);

// privatekey.from_wif
struct BSVPrivateKey* privatekey_from_wif(const char* wif);

// privatekey.from_hex
struct BSVPrivateKey* privatekey_from_hex(const char* hex);

// privatekey.sign_message
struct Signature* privatekey_sign_message(struct BSVPrivateKey* private_key, const uint8_t* msg, size_t msg_len);

// privatekey.to_wif
char* privatekey_to_wif(struct BSVPrivateKey* private_key);

// privatekey.from_bytes
struct BSVPrivateKey* privatekey_from_bytes(const uint8_t* bytes, size_t bytes_len);

// privatekey.to_public_key
struct BSVPublicKey* privatekey_to_public_key(struct BSVPrivateKey* private_key);

// privatekey.encrypt_message
struct ECIESCiphertext* privatekey_encrypt_message(struct BSVPrivateKey* private_key, const uint8_t* message, size_t message_len);

// privatekey.decrypt_message
ByteArray privatekey_decrypt_message(struct BSVPrivateKey* private_key, struct ECIESCiphertext* ciphertext, struct BSVPublicKey* sender_pub_key);