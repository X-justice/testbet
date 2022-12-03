use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::msg;
use solana_program::program_error::ProgramError;
// use solana_program::entrypoint::Result;
use std::convert::TryInto;

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Amount {
    draw: u64,
}

#[derive(Debug)]
pub enum HelloInstruction {
    CreateBet(u64),
    CreateBidTransfer(u64),
    CreateBidApprove(u64),
    DrawAmount(u64),
    AirdropToken
}

impl HelloInstruction {
    pub fn unpack(input: &[u8]) -> Result<HelloInstruction, ProgramError> {
        let (&tag, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        msg!("unpack {:?}", input);
        msg!("tag: {:?}", tag);
        msg!("rest: {:?}", rest);
        // let amount = Amount::try_from_slice(&rest)?;
        // msg!("AMOUNT: {:?}", amount);
        // msg!("value: {:?}", xjust);
        // msg!("side: {:?}", side);
        match tag {
            1 => {
                let amount = Amount::try_from_slice(&rest)?;
                msg!("AMOUNT: {:?}", amount);
                return Ok(HelloInstruction::CreateBet(amount.draw));
            }
            2 => {
                let xjust: Result<[u8; 8], _> = rest[..8].try_into();
                let side: Result<[u8; 1], _> = rest[8..].try_into();
                match (xjust, side) {
                    (Ok(i), Ok(j)) => {
                        return Ok(HelloInstruction::CreateBidTransfer(
                            u64::from_le_bytes(i)
                        ))
                    }
                    _ => return Err(ProgramError::InvalidInstructionData),
                }
            }
            3 => {
                let amount = Amount::try_from_slice(&rest)?;
                // msg!("draw {:?}", draw.clone());
                return Ok(HelloInstruction::DrawAmount(amount.draw));
            }
            4 => {
                let amount = Amount::try_from_slice(&rest)?;
                msg!("AMOUNT: {:?}", amount);
                return Ok(HelloInstruction::CreateBidApprove(amount.draw));
            }
            // test token
            5 => {
                return Ok(HelloInstruction::AirdropToken)
            }

            _ => return Err(ProgramError::InvalidInstructionData),
        }
    }
}
