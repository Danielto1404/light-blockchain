use std::borrow::Borrow;
use serde_json::Value::String;
use crate::account::account::Account;
use crate::chain::block::Block;
use crate::chain::transaction::Transaction;
use crate::env::gas::Gas;
use crate::env::token::FungibleTokenAmount;

pub struct Chain {
    pub blocks: Vec<Block>,
    pub current_transactions: Vec<Transaction>,
    pub difficulty: u32,
    pub miner: Account,
    pub reward: FungibleTokenAmount,
}

impl Chain {
    pub fn new(miner: Account, difficulty: u32, reward: FungibleTokenAmount) -> Chain {
        Chain {
            blocks: Vec::new(),
            current_transactions: Vec::new(),
            difficulty,
            miner,
            reward,
        }
    }

    pub fn create_tx(
        &mut self,
        sender: Account,
        receiver: Account,
        amount: FungibleTokenAmount,
        fee: FungibleTokenAmount,
    ) {
        let transaction = Transaction {
            sender,
            receiver,
            amount,
            gas: Gas {
                fee,
            },
        };

        self.current_transactions.push(transaction)
    }

    pub fn last_block(&self) -> String {
        let block = match self.blocks.last() {
            Some(block) => block,
            _ => return String::from_utf8(vec![48, 64]).unwrap()
        };


        // Chain::hash(b);
    }

    // fn hash(Bl)


    pub fn generate_block() {}
}