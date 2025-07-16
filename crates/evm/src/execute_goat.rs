use alloc::vec::Vec;

use alloy_eips::eip7685::Requests;
use alloy_evm::block::BlockValidationError;
use alloy_primitives::{Bytes, Log, U256};
use anyhow::{Context, Result};
use revm_database::states::state::State;

use crate::goat_types::*;

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

/// Process GOAT requests and return a [`Requests`] container.
pub fn process_goat_requests(
    block_number: u64,
    reward: u128,
    all_logs: Vec<Log>,
) -> Result<Requests> {
    let mut locking_requests = LockingRequests::default();
    let mut bridge_requests = BridgeRequests::default();
    let mut relayer_requests = RelayerRequests::default();

    locking_requests.gas.push(GasRequest::new(block_number, U256::from(reward)));

    for log in all_logs {
        match log.address {
            BITCOINT_CONTRACT => {
                if log.data.topics().len() < 1 {
                    continue;
                }

                match log.data.topics()[0] {
                    WITHDRAW_EVENT_TOPIC => {
                        let req = unpack_into_withdraw_request(log.data.topics(), &log.data.data)
                            .context("Failed to unpack withdraw request")?;
                        bridge_requests.withdraws.push(req);
                    }
                    REPLACE_BY_FEE_EVENT_TOPIC => {
                        let req =
                            unpack_into_replace_by_fee_request(log.data.topics(), &log.data.data)
                                .context("Failed to unpack replace by fee request")?;
                        bridge_requests.replace_by_fees.push(req);
                    }
                    CANCEL1_EVENT_TOPIC => {
                        let req = unpack_into_cancel1_request(log.data.topics(), &log.data.data)
                            .context("Failed to unpack cancel1 request")?;
                        bridge_requests.cancel1s.push(req);
                    }
                    UPDATE_DEPOSIT_TAX_EVENT_TOPIC => {
                        let req = unpack_into_deposit_tax_request(&log.data.data)
                            .context("Failed to unpack deposit tax request")?;
                        bridge_requests.deposit_tax.push(req);
                    }
                    CONFIRMATION_NUMBER_EVENT_TOPIC => {
                        let req = unpack_into_confirmation_number_request(&log.data.data)
                            .context("Failed to unpack confirmation number request")?;
                        bridge_requests.confirmation.push(req);
                    }
                    UPDATE_MIN_DEPOSIT_EVENT_TOPIC => {
                        let req = unpack_into_min_deposit_request(&log.data.data)
                            .context("Failed to unpack min deposit request")?;
                        bridge_requests.min_deposit.push(req);
                    }
                    _ => continue,
                }
            }
            LOCKING_CONTRACT => {
                if log.data.topics().len() != 1 {
                    continue;
                }

                match log.data.topics()[0] {
                    CREATE_EVENT_TOPIC => {
                        let req = unpack_into_create_request(&log.data.data)
                            .context("Failed to unpack create request")?;
                        locking_requests.creates.push(req);
                    }
                    LOCK_EVENT_TOPIC => {
                        let req = unpack_into_lock_request(&log.data.data)
                            .context("Failed to unpack lock request")?;
                        locking_requests.locks.push(req);
                    }
                    UNLOCK_EVENT_TOPIC => {
                        let req = unpack_into_unlock_request(&log.data.data)
                            .context("Failed to unpack unlock request")?;
                        locking_requests.unlocks.push(req);
                    }
                    CLAIM_EVENT_TOPIC => {
                        let req = unpack_into_claim_request(&log.data.data)
                            .context("Failed to unpack claim request")?;
                        locking_requests.claims.push(req);
                    }
                    GRANT_EVENT_TOPIC => {
                        let req = unpack_into_grant_request(&log.data.data)
                            .context("Failed to unpack grant request")?;
                        locking_requests.grants.push(req);
                    }
                    UPDATE_TOKEN_WEIGHT_EVENT_TOPIC => {
                        let req = unpack_into_update_token_weight_request(&log.data.data)
                            .context("Failed to unpack update token weight request")?;
                        locking_requests.update_weights.push(req);
                    }
                    UPDATE_TOKEN_THRESHOLD_EVENT_TOPIC => {
                        let req = unpack_into_update_token_threshold_request(&log.data.data)
                            .context("Failed to unpack update token threshold request")?;
                        locking_requests.update_thresholds.push(req);
                    }
                    _ => continue,
                }
            }
            RELAYER_CONTRACT => {
                if log.data.topics().len() != 2 {
                    continue;
                }

                match log.data.topics()[0] {
                    ADD_VOTER_EVENT_TOPIC => {
                        let req = unpack_into_add_voter_request(log.data.topics(), &log.data.data)
                            .context("Failed to unpack add voter request")?;
                        relayer_requests.adds.push(req);
                    }
                    REMOVE_VOTER_EVENT_TOPIC => {
                        let req =
                            unpack_into_remove_voter_request(log.data.topics(), &log.data.data)
                                .context("Failed to unpack remove voter request")?;
                        relayer_requests.removes.push(req);
                    }
                    _ => continue,
                }
            }
            _ => continue,
        }
    }

    let mut requests = Requests::default();

    locking_requests.encode()?.into_iter().for_each(|req| {
        requests.push_request(Bytes(req.into()));
    });
    bridge_requests.encode()?.into_iter().for_each(|req| {
        requests.push_request(Bytes(req.into()));
    });
    relayer_requests.encode()?.into_iter().for_each(|req| {
        requests.push_request(Bytes(req.into()));
    });

    Ok(requests)
}
