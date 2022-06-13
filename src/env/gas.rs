use crate::env::token::FungibleTokenAmount;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Gas {
    pub fee: FungibleTokenAmount,
}