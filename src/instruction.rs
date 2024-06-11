use solana_program::program_error::ProgramError;
use borsh::BorshDeserialize;

#[derive(BorshDeserialize)]
struct BidPayload {
    bid_type: String,
    energy_units: f64,
    price: f64,
    bid_status:String,
    //TODO: check for the type of wallet
    wallet: String,
    date: String,
    guid: String
}

pub enum BidInstruction {
    AddBid {
        bid_type: String,
        energy_units: f64,
        price: f64,
        bid_status:String,
        //TODO: check for the type of wallet
        wallet: String,
        date: String,
        guid: String
    }
}

impl BidInstruction {
    
    pub fn unpack(data: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = data.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        let payload = BidPayload::try_from_slice(rest).unwrap();

        Ok(match variant {
            0 => Self::AddBid {
                    bid_type: payload.bid_type,
                    energy_units: payload.energy_units,
                    price: payload.price,
                    bid_status:payload.bid_status,
                    wallet: payload.wallet,
                    date: payload.date,
                    guid: payload.guid
                },
            
            _ => return Err(ProgramError::InvalidInstructionData)
        })
    }
}