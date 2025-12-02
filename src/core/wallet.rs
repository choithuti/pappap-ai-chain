// src/core/wallet.rs
use ed25519_dalek::{SigningKey, VerifyingKey, Signer};
use rand::rngs::OsRng;
use bip39::{Mnemonic, Language};
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Wallet {
    pub address: String,
    pub mnemonic: String,
    pub public_key: String,
    // Secret key không bao giờ được serialize ra JSON mặc định để bảo mật
    #[serde(skip)]
    pub secret_key: Vec<u8>,
}

impl Wallet {
    /// Tạo ví mới ngẫu nhiên
    pub fn new() -> Self {
        // 1. Tạo Entropy ngẫu nhiên (32 bytes)
        let mut entropy = [0u8; 32];
        use rand::RngCore;
        OsRng.fill_bytes(&mut entropy);
        
        // 2. Tạo Mnemonic (12 hoặc 24 từ)
        let mnemonic = Mnemonic::from_entropy_in(Language::English, &entropy).unwrap();
        let phrase = mnemonic.words().collect::<Vec<&str>>().join(" ");
        
        // 3. Tạo Keypair từ Seed
        let seed = mnemonic.to_seed(""); // Có thể thêm password vào đây
        let signing_key = SigningKey::from_bytes(&seed[0..32].try_into().unwrap());
        let verifying_key = VerifyingKey::from(&signing_key);
        
        // 4. Tạo địa chỉ PAPPAP
        // Address = PAPPAP + Hex(SHA256(PublicKey)[0..16])
        let mut hasher = Sha256::new();
        hasher.update(verifying_key.to_bytes());
        let address_hash = hex::encode(&hasher.finalize()[0..16]);
        let address = format!("PAPPAP{}", address_hash).to_uppercase();

        println!("🔑 NEW WALLET GENERATED: {}", address);

        Self {
            address,
            mnemonic: phrase,
            public_key: hex::encode(verifying_key.to_bytes()),
            secret_key: signing_key.to_bytes().to_vec(),
        }
    }

    /// Ký tin nhắn (transaction)
    pub fn sign(&self, message: &[u8]) -> String {
        let signing_key = SigningKey::from_bytes(self.secret_key.as_slice().try_into().unwrap());
        let signature = signing_key.sign(message);
        hex::encode(signature.to_bytes())
    }
}
