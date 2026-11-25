# My First Blockchain in Rust

In this project, I coded a simple blockchain in **Rust**. I wanted to understand how cryptocurrencies like Bitcoin actually work under the hood.

## What can this program do?

- **Create blocks:** Data (like transactions) gets saved inside individual blocks.
- **Link them securely:** Every block gets a digital fingerprint (SHA-256 hash) and also stores the fingerprint of the block before it. This connects all the blocks like a chain.
- **Mining (Proof-of-Work):** Before a new block is added, the PC has to solve a small number puzzle.
- **Catch cheating:** The program can check if someone tried to secretly change data in an older block.

## What you need

- You need to have **Rust** and **Cargo** installed on your computer.

## How to run the project

1. **Create a new project:**
   Open your terminal and type:
   ```bash
   cargo new my-blockchain
   cd my-blockchain
   ```

2. **Add the packages:**
   Open the `Cargo.toml` file and add this at the very bottom:
   ```toml
   [dependencies]
   sha2 = "0.10"
   chrono = "0.4"
   ```

3. **Paste the code:**
   Copy the Rust code into the `src/main.rs` file.

4. **Run the program:**
   ```bash
   cargo run --release
   ```
   *(Tip: Using `--release` makes your PC mine way faster!)*

## How does it work?

1. **The starting block:** When the program starts, it automatically builds the very first block (called the *Genesis Block*).
2. **Mining:** When new data is saved, the PC counts a number (`nonce`) up until the block's fingerprint starts with four zeros (`0000...`). That takes a bit of computing power.
3. **The check:** If someone later changes "10 Coins" to "1000 Coins", the block's fingerprint changes too. The chain won't match anymore, and the program prints `false` (invalid).

## Example output

```text
Mine Block 1...
Mine Block 2...
Block {
    index: 0,
    timestamp: 1795537920,
    data: "Genesis Block",
    previous_hash: "0000000000000000000000000000000000000000000000000000000000000000",
    nonce: 41829,
    hash: "0000a8f9c2e1...",
}
...
Blockchain gültig? true
```

## Ideas for later

- Add real wallets (accounts)
- Connect it over the internet to run it with friends
- Save the blockchain to a file so it doesn't disappear when closing the program
