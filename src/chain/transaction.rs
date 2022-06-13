use crate::account::account::{Account, Amount};
use crate::env::gas::Gas;
use crate::env::token::FungibleTokenAmount;

pub struct Transaction {
    pub sender: Account,
    pub receiver: Account,
    pub gas: Gas,
    pub amount: FungibleTokenAmount,
}