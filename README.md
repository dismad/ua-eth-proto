# UA + Ethereum from Seed

Prototype that:

- Accepts a BIP-39 mnemonic **or** generates a fresh 12-word mnemonic
- Derives an Orchard address from the seed (same approach as spvisualizer)
- Derives an Ethereum address from the same seed (`m/44'/60'/0'/0/0`)
- Embeds the Ethereum address in the Unified Address under experimental typecode `0x10`

## Build & Run

```bash
cargo run
# or with a mnemonic:
cargo run -- "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
```

## Dependencies

Uses the same Zcash crates as `spvisualizer` plus BIP-32 / secp256k1 for Ethereum derivation.
