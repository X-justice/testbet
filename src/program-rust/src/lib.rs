use borsh::{BorshDeserialize, BorshSerialize};

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
};

pub mod instruction;
use crate::instruction::HelloInstruction;

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Bid {
    /// XJUST lamports
    pub xjust: u64,
    /// selected side
    pub side: u8,
    /// user key
    pub pubkey: String,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct BidData {
    // list bids
    pub bids: Vec<Bid>
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, 
    accounts: &[AccountInfo], 
    instruction_data: &[u8], 
) -> ProgramResult {
    msg!("Hello World Rust program entrypoint");
    let acc_iter = &mut accounts.iter();
    let user = next_account_info(acc_iter)?;
    let bet = next_account_info(acc_iter)?;
    // let program = next_account_info(acc_iter)?;
    msg!("bet key: {:?}", bet.key);
    let instruction = HelloInstruction::unpack(instruction_data)?;
    msg!("TEST: {:?}", instruction);

    match instruction {
        HelloInstruction::Increment => {}
        HelloInstruction::Decrement => {}

        HelloInstruction::CreateBid(xjust, side) => {
            msg!("value: {:?}", xjust);
            msg!("side: {:?}", side);
            
            let mut bet_account = BidData::try_from_slice(&bet.data.borrow())?;
            let bid = Bid {
                side: side,
                xjust: xjust,
                pubkey: user.key.to_string()
            };
            msg!("bid: {:?}", bid);
            bet_account.bids.push(bid);
         
            bet_account.serialize(&mut &mut bet.data.borrow_mut()[..])?;
            invoke(
                &system_instruction::transfer(user.key, bet.key, xjust),
                &[user.clone(), bet.clone()],
            )?;
        }
    }

    Ok(())
}
