pub mod instruction;
pub mod state;

use instruction::BidInstruction;
use state::BidAccountState;

use borsh::BorshSerialize;
use std::convert::TryInto;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    // borsh::try_from_slice_unchecked,
    borsh::try_from_slice_unchecked,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    pubkey::Pubkey,
    system_instruction,
    // program_error::ProgramError,
    sysvar::{rent::Rent, Sysvar},
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = BidInstruction::unpack(instruction_data)?;
    match instruction {
        BidInstruction::AddBid {
            bid_type,
            energy_units,
            price,
            bid_status,
            wallet,
            date,
            guid,
        } => add_bid_review(
            program_id,
            accounts,
            bid_type,
            energy_units,
            price,
            bid_status,
            wallet,
            date,
            guid,
        ),
    }
}

pub fn add_bid_review(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    bid_type: String,
    energy_units: f64,
    price: f64,
    bid_status:String,
    wallet: String,
    date: String,
    guid: String,
) -> ProgramResult {
    msg!("Adding Bid...");
    msg!("Bid Type: {}", bid_type);
    msg!("Energy Units: {}", energy_units);
    msg!("Price: {}", price);
    msg!("Bid Status: {}", bid_status);
    msg!("Wallet: {}", wallet);

    // Get Account iterator
    let account_info_iter = &mut accounts.iter();

    // Get accounts
    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    // Derive PDA and check that it matches client
    let (pda, bump_seed) = Pubkey::find_program_address(
        &[
            initializer.key.as_ref(),
            date.as_bytes().as_ref(),
            guid.as_bytes().as_ref(),
        ],
        program_id,
    );

    // Calculate account size required
    let account_len: usize = 8 + 8 + (4 + bid_type.len()) + (4 + wallet.len()) +(4 + bid_status.len());

    // Calculate rent required
    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(account_len);

    // Create the account
    invoke_signed(
        &system_instruction::create_account(
            initializer.key,
            pda_account.key,
            rent_lamports,
            account_len.try_into().unwrap(),
            program_id,
        ),
        &[
            initializer.clone(),
            pda_account.clone(),
            system_program.clone(),
        ],
        &[&[
            initializer.key.as_ref(),
            date.as_bytes().as_ref(),
            guid.as_bytes().as_ref(),
            &[bump_seed],
        ]],
    )?;

    msg!("PDA created: {}", pda);

    msg!("unpacking state account");
    let mut account_data =
        try_from_slice_unchecked::<BidAccountState>(&pda_account.data.borrow()).unwrap();
    msg!("borrowed account data");

    account_data.bid_type = bid_type;
    account_data.energy_units = energy_units;
    account_data.price = price;
    account_data.wallet = wallet;

    msg!("serializing account");
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    msg!("state account serialized");

    Ok(())
}
