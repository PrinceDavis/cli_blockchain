use serde::{self, Serialize};
use serde_json;
use std::{time};

#[derive(Debug, Clone, Serialize)]
struct Transaction {
  sender:String,
  reciever: String,
  amount: f32,
}
#[derive(Serialize, Debug)]
pub struct BlockHeader  {
  timestamp: u64,
  nonce: u32,
  pre_hash: String,
  merkle: String,
  difficulty: u32,
}

#[derive(Debug)]
pub struct Block {
  header: BlockHeader,
  count: u32,
  transactions: Vec<Transaction>
}

pub struct Chain {
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

  pub fn new_transaction(&mut self, sender: String, reciever: String, amount: f32) -> bool {
    self.curr_trans.push(Transaction{
      sender,
      reciever,
      amount,
    });
    true
  }

  pub fn last_hash(&self) -> String {
    let block = match self.chain.last() {
        Some(block) => block,
        None => return String::from_utf8(vec![48; 64]).unwrap()
    };
    Chain::hash(&block.header)
  }

  pub fn update_difficulty(&mut self, difficulty: u32) -> bool {
    self.difficulty = difficulty;
    true
  }

  pub fn update_reward(&mut self, reward: f32) -> bool {
    self.reward = reward;
    true
  }

  pub fn generate_new_block(&mut self) -> bool {
    let header = BlockHeader {
      timestamp: time::Instant::now().elapsed().as_secs(),
      nonce: 0,
      pre_hash: self.last_hash(),
      difficulty: self.difficulty,
      merkle: String::new(),
    };

    let reward_transaction = Transaction {
      sender: String::from("Root"),
      reciever: self.miner_addr.clone(),
      amount: self.reward
    };

    let mut block = Block {
      header,
      count: 0,
      transactions: vec![]
    };

    block.transactions.push(reward_transaction);
    block.transactions.append(&mut self.curr_trans);

    block.count = block.transactions.len() as u32;

    block.header.merkle = Chain::get_merkle(block.transactions.clone());
    Chain::proof_of_work(&mut block.header);

    print!("{:?} ", &block);
    self.chain.push(block);

    true
  }

  pub fn get_merkle(curr_trans: Vec<Transaction>) -> String {
    let mut merkel = Vec::new();

    for t in &curr_trans {
      let hash = Chain::hash(t);
      merkel.push(hash)
    }

    if merkel.len() % 2 == 1 {
      let last = merkel.last().cloned().unwrap();
      merkel.push(last);
    }

    while merkel.len() > 1 {
      let mut h1 = merkel.remove(0);
      let mut h2 = merkel.remove(0);

      h1.push_str(&mut h2);
      let nh = Chain::hash(&h1);
      merkel.push(nh)
    }
    merkel.pop().unwrap()
  }

  pub fn proof_of_work(header: &mut BlockHeader) {
    loop {
      let hash = Chain::hash(header);
      let slice = &hash[..header.difficulty as usize];

      match slice.parse::<u32>() {
        Ok(val) => {
          if val != 0 {
            header.nonce += 1;
          }else {
            println!("Block hash: {}", hash);
            break;
          }
        },
        Err(_) => {
          header.nonce += 1;
          continue;
        }
      }
    }
  }

  pub fn hash<T:serde::Serialize>(item: &T) -> String {
    let input = serde_json::to_string(&item).unwrap();
    String::new()
  }

}
