use chrono::Utc;
use sha2::{Digest, Sha256};
use std::fmt::Write;

#[derive(Debug, Clone)]
struct Block {
    index: u64,
    timestamp: i64,
    data: String,
    previous_hash: String,
    nonce: u64,
    hash: String,
}

impl Block {
    fn new(index: u64, data: String, previous_hash: String, difficulty: usize) -> Self {
        let timestamp = Utc::now().timestamp();
        let mut block = Block {
            index,
            timestamp,
            data,
            previous_hash,
            nonce: 0,
            hash: String::new(),
        };
        block.mine(difficulty);
        block
    }

    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        let input = format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, self.data, self.previous_hash, self.nonce
        );
        hasher.update(input.as_bytes());
      
        let result = hasher.finalize();
        let mut hex_string = String::with_capacity(64);
        for byte in result {
            write!(&mut hex_string, "{:02x}", byte).unwrap();
        }
        hex_string
    }

    fn mine(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        loop {
            self.hash = self.calculate_hash();
            if self.hash.starts_with(&target) {
                break;
            }
            self.nonce += 1;
        }
    }
}

struct Blockchain {
    chain: Vec<Block>,
    difficulty: usize,
}

impl Blockchain {
    fn new(difficulty: usize) -> Self {
        let genesis_block = Block::new(0, "Genesis Block".to_string(), "0".repeat(64), difficulty);
        Blockchain {
            chain: vec![genesis_block],
            difficulty,
        }
    }

    fn add_block(&mut self, data: String) {
        let previous_block = self.chain.last().unwrap();
        let new_block = Block::new(
            previous_block.index + 1,
            data,
            previous_block.hash.clone(),
            self.difficulty,
        );
        self.chain.push(new_block);
    }
  
    fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            if current.hash != current.calculate_hash() {
                return false;
            }
            if current.previous_hash != previous.hash {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut my_chain = Blockchain::new(4);

    println!("Mining Block 1...");
    my_chain.add_block("Alice sends 10 Coins to Bob".to_string());

    println!("Mining Block 2...");
    my_chain.add_block("Bob sends 5 Coins to Charlie".to_string());

    for block in &my_chain.chain {
        println!("{:#?}", block);
    }

    println!("Is blockchain valid? {}", my_chain.is_valid());
}
