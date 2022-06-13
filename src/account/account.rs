use crate::env::token::FungibleTokenAmount;

pub type AccountId = String;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub address: AccountId,
    pub balance: FungibleTokenAmount,
}