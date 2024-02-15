use anchor_lang::solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    program_pack::{IsInitialized, Pack, Sealed},
    sysvar::{rent::Rent, Sysvar},
};

#[derive(Debug, PartialEq)]
struct AresToken {
    is_initialized: bool,
    supply: u64,
    ares_symbol: [u8; 4], // 4-byte symbol, e.g., "ARES"
}

impl Sealed for AresToken {}

impl IsInitialized for AresToken {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for AresToken {
    const LEN: usize = 13; // 1 (is_initialized) + 8 (supply) + 4 (ares_symbol)

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let is_initialized = src[0] != 0;
        let supply = u64::from_le_bytes(src[1..9].try_into().unwrap());
        let mut ares_symbol = [0u8; 4];
        ares_symbol.copy_from_slice(&src[9..13]);

        Ok(AresToken { is_initialized, supply, ares_symbol })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        dst[0] = self.is_initialized as u8;
        dst[1..9].copy_from_slice(&self.supply.to_le_bytes());
        dst[9..13].copy_from_slice(&self.ares_symbol);
    }
}

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("Ares Token program entrypoint");

    let accounts_iter = &mut accounts.iter();

    let ares_account = next_account_info(accounts_iter)?;

    if ares_account.owner != program_id {
        msg!("Ares account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    let rent = &Rent::from_account_info(next_account_info(accounts_iter)?)?;
    if !ares_account.is_rent_exempt(rent) {
        msg!("Ares account is not rent exempt");
        return Err(ProgramError::AccountNotRentExempt);
    }

    let mut ares_token_data = AresToken::unpack_from_slice(&ares_account.data.borrow())?;
    if ares_token_data.is_initialized {
        msg!("Ares Token is already initialized");
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    ares_token_data.is_initialized = true;
    ares_token_data.supply = 40_000_000 * 1_000_000; // 40M tokens with 6 decimal places
    ares_token_data.ares_symbol = *b"ARES"; // 4-byte symbol

    AresToken::pack_into_slice(&ares_token_data, &mut ares_account.data.borrow_mut());

    Ok(())
}
