# ua-eth-proto

Prototype that derives a Zcash Orchard address and an Ethereum address from the same BIP-39 seed, then embeds the Ethereum address inside a Unified Address under an experimental typecode.


![img](https://github.com/user-attachments/assets/e9c74f7a-a430-4b28-b0a8-c65081ab1feb)



## What it does

- Accepts a BIP-39 mnemonic or generates a fresh 24-word mnemonic
- Derives an Ethereum address (`m/44'/60'/0'/0/0`)
- Derives an Orchard address from the same seed
- Builds a Unified Address containing:
  - the Orchard receiver
  - an experimental item (typecode `0x10`) holding the Ethereum address
- Prints QR codes for both addresses
- The UA QR is labeled with a magenta `[contains ETH address]` badge

A normal Zcash wallet ignores the experimental item and pays the Orchard receiver.  
A Farcaster-aware scanner can read the UA QR and extract the Ethereum address.

## Build

```bash
cargo build -r
```

## Run

Generate a fresh 24-word seed:

```bash
./target/release/ua-eth-seed
```

Use an existing mnemonic:

```bash
./target/release/ua-eth-seed "word1 word2 ... word24"
```

## Output

- Generated or provided mnemonic
- Seed preview
- Ethereum address
- Orchard raw address
- QR code for the Ethereum address
- QR code for the Unified Address (labeled `[contains ETH address]`)
- Round-trip confirmation that the Ethereum address was recovered

## Notes

- Typecode `0x10` is experimental and not standardized.
- The Ethereum address is derived independently from the Zcash keys; they share only the seed.
- This is a prototype for exploring Farcaster + Zcash identity linking.
