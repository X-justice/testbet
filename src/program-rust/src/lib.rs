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
    pub bids: Vec<Bid>,
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
    let instruction = HelloInstruction::unpack(instruction_data)?;
    msg!("TEST: {:?}", instruction);
    msg!("???");

    match instruction {
        HelloInstruction::CreateBet(amount) => {
            let acc_iter = &mut accounts.iter();
            let user = next_account_info(acc_iter)?;
            let bet = next_account_info(acc_iter)?;
            msg!("amount: {:?}", amount);
            invoke(
                &system_instruction::transfer(user.key, bet.key, amount),
                &[user.clone(), bet.clone()],
            )?;
        }



        HelloInstruction::CreateBidApprove(xjust) => {
            let acc_iter = &mut accounts.iter();
            let user = next_account_info(acc_iter)?;
            let user_token = next_account_info(acc_iter)?;
            let bet_token = next_account_info(acc_iter)?;
            let token_info = next_account_info(acc_iter)?;
            msg!("value(XJUST): {:?}", xjust);
            let tx = spl_token::instruction::approve(
                token_info.key, 
                user_token.key,
                bet_token.key,
                user.key,
                &[user.key],
                xjust
            )?;
            invoke(
                &tx,
                &[
                    user_token.clone(),
                    bet_token.clone(),
                    user.clone(),
                    token_info.clone()
                ],
            )?;
        }

        HelloInstruction::CreateBidTransfer(xjust) => {
            let acc_iter = &mut accounts.iter();
            let user = next_account_info(acc_iter)?;
            let user_token = next_account_info(acc_iter)?;
            let bet_token = next_account_info(acc_iter)?;
            let token_info = next_account_info(acc_iter)?;
            msg!("value(XJUST): {:?}", xjust);
            let tx = spl_token::instruction::transfer(
                token_info.key, 
                user_token.key,
                bet_token.key,
                user.key,
                &[user.key],
                xjust
            )?;
            invoke(
                &tx,
                &[
                    user_token.clone(),
                    bet_token.clone(),
                    user.clone(),
                    token_info.clone()
                ],
            )?;
        }

        HelloInstruction::DrawAmount(draw) => {
            let acc_iter = &mut accounts.iter();
            let bet = next_account_info(acc_iter)?;
            let bet_token = next_account_info(acc_iter)?;
            let user_token = next_account_info(acc_iter)?;
            let token = next_account_info(acc_iter)?;
            let tx = spl_token::instruction::transfer(
                token.key, 
                bet_token.key,
                user_token.key,
                bet.key,
                &[bet.key],
                draw
            )?;
            invoke(
                &tx,
                &[
                    bet_token.clone(),
                    user_token.clone(),
                    bet.clone(),
                    token.clone()
                ],
            )?;
            msg!("user: {:?} draw: {:?}", user_token.key, draw);
        }

        HelloInstruction::AirdropToken => {
            let acc_iter = &mut accounts.iter();
            let handler = next_account_info(acc_iter)?;
            let handler_token = next_account_info(acc_iter)?;
            let user_token = next_account_info(acc_iter)?;
            let token = next_account_info(acc_iter)?;
            let tx = spl_token::instruction::transfer(
                token.key, 
                handler_token.key,
                user_token.key,
                handler.key,
                &[handler.key],
                100_000_000
            )?;
            invoke(
                &tx,
                &[
                    handler_token.clone(),
                    user_token.clone(),
                    handler.clone(),
                    token.clone()
                ],
            )?;
            msg!("airdrop token to: {:?} draw: {:?}", user_token.key, 100_000_000);
        }
    }

    Ok(())
}
