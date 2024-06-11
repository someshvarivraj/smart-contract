use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct BidAccountState {
    pub bid_type: String,
    pub energy_units: f64,
    pub price: f64,
    pub bid_status: String,
    //TODO: check for the type of wallet
    pub wallet: String,
}
