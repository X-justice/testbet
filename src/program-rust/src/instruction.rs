use solana_program::msg;
use solana_program::program_error::ProgramError;
// use solana_program::entrypoint::Result;
use std::convert::TryInto;
#[derive(Debug)]
pub enum HelloInstruction {
    Increment,
    Decrement,
    CreateBid(u64, u8),
}

impl HelloInstruction {
    pub fn unpack(input: &[u8]) -> Result<HelloInstruction, ProgramError> {
        let (&tag, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        
        let xjust: Result<[u8; 8], _> = rest[..8].try_into();
        let side: Result<[u8; 1],_> = rest[8..].try_into();
        msg!("unpack {:?}", input);
        msg!("tag: {:?}", tag);
        msg!("rest: {:?}", rest);
        msg!("value: {:?}", xjust);
        msg!("side: {:?}", side);
        
        match tag {
            0 => return Ok(HelloInstruction::Increment),
            1 => return Ok(HelloInstruction::Decrement),
            2 => {
                
                match (xjust, side) {
                    (Ok(i), Ok(j)) => return Ok(HelloInstruction::CreateBid(u64::from_le_bytes(i), u8::from_le_bytes(j))),
                    _ => return Err(ProgramError::InvalidInstructionData),
                }
            }
            _ => return Err(ProgramError::InvalidInstructionData),
        }
    }
}
