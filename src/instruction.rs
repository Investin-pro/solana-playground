use solana_program::{
    msg,
    program_error::ProgramError,
    pubkey::{Pubkey, PUBKEY_BYTES},
};

use crate::error::SwapError;

pub enum SwapInstruction {
    Initialize { amount: u64 },

    Swap { amount: u64 },
}

impl SwapInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (x, rest) = input
            .split_first()
            .ok_or(SwapError::InstructionUnpackError)?;
        Ok(match x {
            0 => {
                let (amount, _) = Self::unpack_u64(rest)?;
                Self::Initialize { amount }
            }
            1 => {
                let (amount, _) = Self::unpack_u64(rest)?;
                Self::Swap { amount }
            }
            _ => {
                msg!("Instruction cannot be unpacked");
                return Err(SwapError::InstructionUnpackError.into());
            }
        })
    }

    fn unpack_pubkey(input: &[u8]) -> Result<(Pubkey, &[u8]), ProgramError> {
        if input.len() < PUBKEY_BYTES {
            msg!("Pubkey cannot be unpacked");
            return Err(SwapError::InstructionUnpackError.into());
        }
        let (key, rest) = input.split_at(PUBKEY_BYTES);
        let pk = Pubkey::new(key);
        Ok((pk, rest))
    }

    fn unpack_u64(input: &[u8]) -> Result<(u64, &[u8]), ProgramError> {
        if input.len() < 8 {
            msg!("u64 cannot be unpacked");
            return Err(SwapError::InstructionUnpackError.into());
        }
        let (bytes, rest) = input.split_at(8);
        let value = bytes
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(SwapError::InstructionUnpackError)?;
        Ok((value, rest))
    }
}
