use bip39::{Mnemonic, Language};
use bip32::{DerivationPath, XPrv};
use hex;
use orchard::keys::{SpendingKey, FullViewingKey, Scope};
use qrcode::{QrCode, render::unicode};
use secp256k1::{Secp256k1, SecretKey};
use sha3::{Digest, Keccak256};
use std::{env, process};
use zcash_protocol::consensus::NetworkType;
use zcash_address::{
    unified::{self, Container, Encoding, Receiver},
};
use zip32::AccountId;

const ETH_TYPECODE: u32 = 0x10;
const ZCASH_COIN_TYPE: u32 = 133;

fn derive_eth_address(seed: &[u8]) -> String {
    let path: DerivationPath = "m/44'/60'/0'/0/0".parse().unwrap();
    let xprv = XPrv::derive_from_path(seed, &path).expect("ETH derivation failed");
    let secret = SecretKey::from_slice(xprv.to_bytes().as_slice()).expect("Invalid secp256k1 key");

    let secp = Secp256k1::new();
    let public = secret.public_key(&secp);
    let uncompressed = public.serialize_uncompressed();
    let hash = Keccak256::digest(&uncompressed[1..]);
    let addr = &hash[12..];
    format!("0x{}", hex::encode(addr))
}

fn derive_orchard_raw_address(seed: &[u8]) -> [u8; 43] {
    let account = AccountId::try_from(0u32).expect("Valid account");
    let sk = SpendingKey::from_zip32_seed(seed, ZCASH_COIN_TYPE, account)
        .expect("Failed to derive Orchard spending key");
    let fvk: FullViewingKey = (&sk).into();
    fvk.address_at(0u32, Scope::External).to_raw_address_bytes()
}

fn print_qr(label: &str, data: &str, badge: Option<&str>) {
    println!("\n{}", label);
    let code = QrCode::new(data.as_bytes()).expect("Failed to generate QR");
    let image = code.render::<unicode::Dense1x2>().build();

    let width = image.lines().map(|l| l.chars().count()).max().unwrap_or(0);
    println!("{}", image);

    if let Some(text) = badge {
        let badge_text = format!("[{}]", text);
        let badge_len = badge_text.chars().count();
        let pad = width.saturating_sub(badge_len) / 2;
        println!("{}\x1b[35m{}\x1b[0m", " ".repeat(pad), badge_text);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let (_mnemonic_str, seed) = if args.len() == 2 {
        let input = &args[1];
        let mnemonic = Mnemonic::parse_in(Language::English, input)
            .unwrap_or_else(|e| {
                eprintln!("Invalid mnemonic: {}", e);
                process::exit(1);
            });
        let seed = mnemonic.to_seed("");
        (input.clone(), seed)
    } else if args.len() == 1 {
        let mut entropy = [0u8; 32];
        getrandom::getrandom(&mut entropy).expect("RNG failed");
        let mnemonic = Mnemonic::from_entropy_in(Language::English, &entropy)
            .expect("Failed to generate mnemonic");
        let seed = mnemonic.to_seed("");
        let phrase = mnemonic.to_string();
        println!("Generated fresh mnemonic:\n{}\n", phrase);
        (phrase, seed)
    } else {
        eprintln!("Usage: {} [mnemonic]", args[0]);
        eprintln!("  No argument  → generate a fresh 24-word seed");
        eprintln!("  With mnemonic → use that seed");
        process::exit(1);
    };

    println!("Seed (first 16 bytes): {}...", hex::encode(&seed[..16]));

    let eth_address = derive_eth_address(&seed);
    println!("Ethereum address: {}", eth_address);

    let orchard_raw = derive_orchard_raw_address(&seed);
    println!("Orchard raw address: {}", hex::encode(orchard_raw));

    let eth_bytes = hex::decode(&eth_address[2..]).unwrap();
    let receivers = vec![
        Receiver::Orchard(orchard_raw),
        Receiver::Unknown {
            typecode: ETH_TYPECODE,
            data: eth_bytes,
        },
    ];

    let ua = unified::Address::try_from_items(receivers)
        .expect("Failed to construct UA");

    let encoded = ua.encode(&NetworkType::Main);

    print_qr("Ethereum address QR:", &eth_address, None);
    print_qr("Zcash UA QR: \x1b[35m[contains ETH address]\x1b[0m", &encoded, None);

    let (_net, decoded) = unified::Address::decode(&encoded).expect("Decode failed");
    for item in decoded.items() {
        match item {
            Receiver::Orchard(_) => println!("  ✓ Orchard receiver present"),
            Receiver::Unknown { typecode, data } if typecode == ETH_TYPECODE => {
                println!("  ✓ ETH address recovered: 0x{}", hex::encode(data));
            }
            _ => {}
        }
    }
}