# Farcaster + Zcash: Embedding an Ethereum Address in a Unified Address

## Goal

Allow a Farcaster client to natively recognize an Ethereum address inside a Zcash Unified Address (UA).  
Since Farcaster identity is based on Ethereum addresses (FID custody), a client can treat the embedded address as the user’s Farcaster identity while still offering a normal Zcash payment destination.

## Why this works

- ZIP 316 Unified Addresses support unknown / experimental typecodes.
- Standard Zcash wallets ignore unrecognized typecodes and still pay the shielded (or transparent) receiver.
- A Farcaster app can specially interpret a chosen experimental typecode and extract the 20-byte Ethereum address.
- Everything can be derived deterministically from a single BIP-39 seed.

## Derivation overview

| Purpose                  | Path / method                          | Output                     |
|--------------------------|----------------------------------------|----------------------------|
| Ethereum (FID custody)   | `m/44'/60'/account'/0/index`           | Ethereum address           |
| Zcash transparent        | `m/44'/133'/account'/0/index`          | t1 address                 |
| Zcash Orchard            | ZIP-32 `m/32'/133'/account'`           | Orchard address            |
| Embedded in UA           | Experimental typecode `0x10`           | Same Ethereum address      |

## Prototype behavior

1. Construct a UA containing at least one valid Zcash receiver (Orchard shown).
2. Add an experimental item with typecode `0x10` holding a 20-byte Ethereum address.
3. Encode to a `u1…` string.
4. Decode and recover both the Orchard receiver and the original Ethereum address.

Standard Zcash software will ignore typecode `0x10`.  
A Farcaster client that understands the typecode can link the UA directly to the corresponding FID.

## Limitations

- Typecode `0x10` is not standardized. It is an application-level convention.
- No existing Zcash wallet will display or use the Ethereum address. Only the Farcaster app will.

## Future possibilities

- Formal ZIP defining a metadata or receiver typecode for an Ethereum address.
- Include a short version/prefix byte in the payload for forward compatibility.
- Derive the Ethereum address and all Zcash keys from the same seed inside the app.


## Next steps for a Farcaster client

1. Derive the Ethereum address from the user’s seed (BIP-44 coin type 60).
2. Derive a real Orchard (or Sapling) address from the same seed.
3. Embed the Ethereum address under typecode `0x10` when building the UA.
4. On receipt of a UA, check for typecode `0x10` and treat the 20 bytes as an Ethereum address / FID lookup.
5. Fall back to normal Zcash payment behavior for the shielded receiver.

This keeps private keys local, requires no protocol changes, and lets the Farcaster app treat the UA as a dual-purpose identity + payment string.
