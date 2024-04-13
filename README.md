<img src="https://github.com/s473573/rust-capstone/assets/115479231/ecc3eba2-abb0-41a8-aa5c-ed8d89a368a2" width="100" alt="">

# Stool: a Steganography Tool 🎭

**Stool** is a [Work In Progress] steganography tool that enables you to hide your most private secrets within the funny pictures you find on the interwebs.
Built in Rust to provide you that sweet performance and safety.

## Features

- **Command-line Interface**: Easily embed text into images in your terminal, or hook it up in a shell script!
- **Encryption**: Secure your secret providing a passphrase. _ChaCha20_ cipher ready to serve your crypto-anarchic needs.

## How to Use Stool

1. **Clone the Repository:**
   ```bash
   git clone https://github.com/s473573/rust-capstone.git
   ```

2. **Navigate to the Project Directory:**
   ```bash
   cd rust-capstone
   ```

3. **Compile the Project:**
   ```bash
   cargo build --release
   ```

3. **Run it!:**
   ```bash
   cargo run -- -k "mypass" insert image.png output.png "literally a secret"
   ```
## Upcoming Features
- Additional image steganography methods: like that clever one which uses JPEG pixel block representation
- Different steganography formats: making possible to hide data inside other general data!
- GUI Interface: making the tool accessible to everyone.
