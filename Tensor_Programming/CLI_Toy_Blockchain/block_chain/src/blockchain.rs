extern crate time;
extern crate serde;
extern crate serde_json;
extern crate sha2;

use sha2::{ Sha256, Digest };
use std::fmt::Write;

#[derive(Debug, Clone, Serialize)]
struct Transnaction{
    sender: String,
    reciever: String,
    amount: f32,
}

#[derive(Debug, Serialize)]
pub struct Blockheader{
    timestamp: i64,
    nonce: u32,
    pre_hash: String,
    merkle: String,
    difficulty: u32,
}

#[derive(Debug, Serialize)]
pub struct Block {
    header: Blockheader,
    count: u32,
    transactions: Vec<Transaction>,
}

pub struct Chain{
    chain: Vec<Block>,
    curr_trans: Vec<Transaction>,
    difficulty: u32,
    miner_addr: String,
    reward: f32,
}

impl Chain {
    pub fn new(miner_addr: String, difficulty: u32) -> Chain {
        let mut chain = Chain{
            chain: Vec::new(),
            curr_trans: Vec::new(),
            difficulty,
            miner_addr,
            reward: 100.0,
        };
        
        chain.generate_new_block();
        chain
    }
   
    pub fn generate_new_block(&mut self, sender: String, reciever: String, amount: f32) -> bool{
        self.curr_trans.push(Transaction {
            sender, reciever, amountd,
        });

        true
    }

    pub fn last_hash(&self) -> String {
        let block= match self.chain.last() {
            Some(block) => block,
            None => return String::from_utf8(vec![48; 64]).unwrap();
        };
        Chain::hash(&block.header)
    }

    pub fn update_difficulty(&mut self, difficulty: u32) -> bool {
        self.difficulty= difficulty;
        true
    }

    pub fn update_reward(&mut self, reward: f32) -> bool{
        self.reward= reward;
        true
    }

    pub fn generate_new_block(&mut self) -> bool {
        let header= Blockheader{
            timestamp: time::now().to_timespec().sec,
        }
    }
}