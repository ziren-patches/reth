use alloy_evm::block::BlockValidationError;
use alloy_primitives::{address, Address};
use revm_database::states::state::State;

const GOAT_FOUNDDATION_CONTRACT: Address = address!("0xbc10000000000000000000000000000000000002");
const GOAT_LOCKING_CONTRACT: Address = address!("0xbc10000000000000000000000000000000000004");

const GF_BASE_POINT: u128 = 200;
const GF_MAX_BASE_POINT: u128 = 10000;

/// The foundation account collects a 2% tax.
/// Allocate remaining fees to locking contract.
pub fn allocate_goat_gas_fees<DB: revm::Database>(
    state_db: &mut State<DB>,
    goat_gas_fees: u128,
) -> Result<u128, BlockValidationError> {
    if goat_gas_fees == 0 {
        return Ok(0);
    }

    // foundation tax 2%
    let tax = goat_gas_fees
        .saturating_mul(GF_BASE_POINT)
        .checked_div(GF_MAX_BASE_POINT)
        .unwrap_or_default();

    if tax > 0 {
        state_db
            .increment_balances([(GOAT_FOUNDDATION_CONTRACT, tax)])
            .map_err(|_| BlockValidationError::IncrementBalanceFailed)?;
    }

    let goat_gas_fees = goat_gas_fees.saturating_sub(tax);
    if goat_gas_fees > 0 {
        state_db
            .increment_balances([(GOAT_LOCKING_CONTRACT, goat_gas_fees)])
            .map_err(|_| BlockValidationError::IncrementBalanceFailed)?;
    }
    Ok(goat_gas_fees)
}
