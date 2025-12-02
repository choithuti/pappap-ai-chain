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
    #[serde(skip)] pub secret_key: Vec<u8>,
}

impl Wallet {
    pub fn new() -> Self {
        let mut entropy = [0u8; 32];
        use rand::RngCore;
        OsRng.fill_bytes(&mut entropy);
        let mnemonic = Mnemonic::from_entropy_in(Language::English, &entropy).unwrap();
        
        let seed = mnemonic.to_seed("");
        let signing_key = SigningKey::from_bytes(&seed[0..32].try_into().unwrap());
        let verifying_key = VerifyingKey::from(&signing_key);
        
        // Tạo địa chỉ bắt đầu bằng PAPPAP
        let mut hasher = Sha256::new();
        hasher.update(verifying_key.to_bytes());
        let address = format!("PAPPAP{}", hex::encode(&hasher.finalize()[0..16])).to_uppercase();

        Self {
            address,
            mnemonic: mnemonic.words().collect::<Vec<&str>>().join(" "),
            public_key: hex::encode(verifying_key.to_bytes()),
            secret_key: signing_key.to_bytes().to_vec(),
        }
    }
}
