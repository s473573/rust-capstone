<img src="https://github.com/s473573/rust-capstone/assets/115479231/ecc3eba2-abb0-41a8-aa5c-ed8d89a368a2" width="100" alt="">

# stool: Steganography Tool 🎭
_A tiny Rust stego + crypto CLI for hiding encrypted data inside images._

![CI](https://github.com/s473573/rust-capstone/actions/workflows/ci.yml/badge.svg)

stool (“steg tool”) lets you hide arbitrary text inside ordinary-looking images using LSB (least significant bit) steganography, optionally encrypted with a passphrase.

Goal: small, auditable privacy tooling for people who care about autonomy, censorship resistance, and not leaving obvious plaintext around.

---

## Status
**Usable.**

- `insert` works: embed a secret into an image.
- `extract` works: recover that secret back out.
- Payloads can be optionally encrypted (ChaCha20).
- Project includes integration-style tests.

Rust edition: 2021+

---

## Why this exists
Most “private” channels still scream “this is encrypted data.” Steganography is different: you blend a secret into what looks like a normal image.

This project explores:
- embedding data into pixel LSBs
- protecting that data with modern symmetric crypto (ChaCha20 stream cipher)
- doing all of that in safe Rust, with zero `unsafe` in the core logic

---

## Features (current)

- **CLI-first workflow**  
  Hide messages in images from the terminal or a script.

- **ChaCha20 encryption (key from passphrase)**  
  Even if someone manages to pull bytes out of the image, they still get ciphertext unless they know your passphrase.

- **Lossless LSB steganography**  
  The secret is embedded in the least-significant bits of pixel channels. Visually the image should look identical to the cover.

- **`insert` and `extract` subcommands**  
  You can both hide and recover data end-to-end.

- **Optional file output / stdout-friendly**  
  `extract` can either write recovered bytes to a file (`--out`) or just print them to stdout. This makes it usable in scripts, and lets you avoid leaving secrets on disk if you don't want to.

- **Logging / observability via `tracing`**  
  Structured logs and `RUST_LOG=debug` support for debugging the pipeline.

- **Tests**  
  Roundtrip tests (encrypt → embed → extract → decrypt) to prove correctness and catch regressions.

---

## Roadmap / Upcoming

- **Capacity reporting**  
  Get a “this cover image can carry N bytes safely” estimate.

- **Multiple carriers**  
  PNG is working. JPEG via DCT-domain tricks is on the roadmap. Audio/video carriers are future-friendly targets.

- **Better passphrase handling**  
  Move from a basic passphrase → key derivation to a hardened KDF (e.g. Argon2id) with per-message salt and sane iteration cost.

- **Simple GUI**  
  A small desktop UI so non-technical users can hide/recover data without touching a terminal.

---

## Install / Build

### Step 1: Clone the repo
```bash
git clone https://github.com/s473573/rust-capstone.git
cd rust-capstone

### Step 2: Build (optimized)
```bash
cargo build --release

## Usage

### Hide a secret inside an image
```bash
cargo run -- \
  -k "mypass" \
  insert input.png output.png "literally a secret"
```

Arguments:
- `-k "mypass"`: passphrase to derive the symmetric key
- `insert`: subcommand to hide data
- `input.png`: cover image
- `output.png`: stego image (written)
- `"literally a secret"`: message to embed (will be encrypted first if `-k` is provided)

### Extract a secret from a stego image (print to stdout)
```bash
cargo run -- \
  -k "mypass" \
  extract output.png
```

If the embedded payload was encrypted with a passphrase, provide the same `-k` when extracting.  
If it's valid UTF-8, the secret is printed directly to stdout.  
If it's arbitrary bytes, it's printed as base64.

### Extract a secret and write it to a file instead
```bash
cargo run -- \
  -k "mypass" \
  extract output.png \
  --out recovered.bin
```

Arguments:
- `extract`: subcommand to recover data
- `output.png`: stego image containing the secret
- `--out recovered.bin` (optional): write raw bytes to a file instead of printing

---

## CLI Help
```bash
cargo run -- --help
cargo run -- insert --help
cargo run -- extract --help
```

---

## Observability (debug / tracing)

The binary uses `tracing` + `tracing_subscriber`, so you can control verbosity:

```bash
RUST_LOG=debug cargo run -- insert ...
RUST_LOG=trace cargo test
```

This is helpful when you’re trying to see exactly how bits are being packed.  
For privacy, the tool avoids logging your actual plaintext secret or your passphrase.

---

## Security / Threat Model

- This tool does **not claim to defeat** a determined forensic steganalysis.  
  A motivated adversary doing statistical analysis of pixel LSBs may still detect anomalies.

- ChaCha20 protects confidentiality of the payload even if the bytes are extracted, **but**:
  - passphrase strength matters,
  - key derivation matters,
  - nonce reuse matters.

- If you are a journalist / activist in immediate danger:  
  Treat this as an educational prototype, not production-grade OPSEC.

---

## Testing

The repo includes tests that:
- generate synthetic noise images at runtime,
- embed known secrets,
- extract them back out,
- verify roundtrip equality,
- exercise the real CLI (`cargo_bin("stool")`) using temporary dirs.

This gives us confidence that embedding/extraction + crypto stays correct as the code evolves.

Run:
```bash
cargo test
```

---

## Tech Stack

- Rust (safe code, no `unsafe` in core steg logic)
- `image` crate for pixel buffer manipulation
- ChaCha20 stream cipher for symmetric encryption
- `tracing` for structured logging
- `assert_cmd`, `assert_fs` for integration-ish tests

---

## License

This project is open source. See `LICENSE` in the repo.

---
