//! Program-scoped slippage errors. Codes come from the checked-in DEX IDLs
//! in onchain_programs_common/idls; identical Anchor codes are NOT global.
use solana_address::{Address, address};

pub const MESSAGE: &str = "Slippage exceeded";

pub const SLIPPAGE_ERRORS: &[(Address, &[u32])] = &[
    (crate::consts::AURA_PROGRAM, &[0]),
    (address!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P"), &[6002, 6003, 6042]),
    (address!("pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA"), &[6004, 6040]),
    (address!("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8"), &[30]),
    (address!("CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C"), &[6005]),
    (address!("CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK"), &[6017, 6018, 6019]),
    (address!("LanMV9sAd7wArD4vJFi2qDdfnVhFxYSUg6eADduJ3uj"), &[6004]),
    (address!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"), &[6003, 6004]),
    (address!("dbcij3LWUppWqq96dh6gJWwBifmcGfLSB5D4DuSMaqN"), &[6002]),
    (address!("cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG"), &[6002]),
    (address!("Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB"), &[6004]),
    (address!("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc"), &[6036, 6037, 6069]),
    (address!("HpNfyc2Saw7RKkQd8nEL4khUcuPhQ7WwY1B2qjx8jxFq"), &[6021, 6022, 6023]),
];

pub fn is_slippage(program: &Address, code: u32) -> bool {
    SLIPPAGE_ERRORS.iter().any(|(id, codes)| id == program && codes.contains(&code))
}

/// Resolve a propagated Aura error from its final direct CPI subtree. Earlier
/// successful hops must not be included. With no runtime logs, unknown nested
/// programs are ambiguous: leave those errors unclassified instead of guessing.
pub fn cpi_slippage_program<'a>(code: u32, mut programs: impl Iterator<Item = &'a Address>) -> Option<Address> {
    let program = *programs.next()?;
    if program == crate::consts::AURA_PROGRAM || !is_slippage(&program, code) {
        return None;
    }
    let unambiguous = programs.all(|nested| {
        *nested == program
            // Standard token/system/memo errors cannot be Anchor 6000+ codes.
            || (code >= 6000 && [
                address!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                address!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
                address!("11111111111111111111111111111111"),
                address!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"),
            ].contains(nested))
    });
    unambiguous.then_some(program)
}

#[cfg(feature = "api-types")]
impl crate::types::TransactionErrorWithAccount {
    pub fn is_slippage(&self) -> bool {
        use solana_instruction_error::InstructionError;
        use solana_transaction_error::TransactionError;
        match (&self.account, &self.err) {
            (Some(program), TransactionError::InstructionError(_, InstructionError::Custom(code))) => {
                is_slippage(program, *code)
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codes_are_scoped_to_the_program() {
        for (program, codes) in SLIPPAGE_ERRORS {
            for code in *codes {
                assert!(is_slippage(program, *code));
                assert!(!is_slippage(&Address::new_from_array([99; 32]), *code));
            }
            assert!(!is_slippage(program, 9999));
        }
        let launchlab = SLIPPAGE_ERRORS[6].0;
        assert!(!is_slippage(&launchlab, 6002));
        assert!(!is_slippage(&crate::consts::AURA_PROGRAM, 6002));
    }

    #[test]
    fn propagated_errors_require_an_unambiguous_final_cpi_subtree() {
        let damm = address!("cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG");
        let token = address!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
        assert_eq!(cpi_slippage_program(6002, [damm, token, damm].iter()), Some(damm));
        assert_eq!(cpi_slippage_program(6004, [damm].iter()), None);
        assert_eq!(cpi_slippage_program(6002, [damm, Address::new_from_array([99; 32])].iter()), None);
        let amm = SLIPPAGE_ERRORS[3].0;
        assert_eq!(cpi_slippage_program(30, [amm].iter()), Some(amm));
        assert_eq!(cpi_slippage_program(30, [amm, token].iter()), None);
        assert_eq!(cpi_slippage_program(6002, core::iter::empty()), None);
    }
}
