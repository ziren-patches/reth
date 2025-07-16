use alloc::{string::String, vec, vec::Vec};

use alloy_primitives::{B256, U256};
use anyhow::{bail, Context, Result};

use super::request::*;

const WITHDRAWAL_REQ_ADDR_LOC: u32 = 128;
const SATOSHI: u64 = 10_000_000_000;
const MAX_TAX_RATE: u32 = 10_000;

#[derive(Debug, Clone, Default)]
pub struct BridgeRequests {
    pub withdraws: Vec<WithdrawalRequest>,
    pub replace_by_fees: Vec<ReplaceByFeeRequest>,
    pub cancel1s: Vec<Cancel1Request>,
    pub deposit_tax: Vec<DepositTaxRequest>,
    pub confirmation: Vec<ConfirmationNumberRequest>,
    pub min_deposit: Vec<MinDepositRequest>,
}

impl BridgeRequests {
    pub fn encode(&self) -> Result<Vec<Vec<u8>>> {
        let mut res = Vec::new();

        if !self.withdraws.is_empty() {
            let mut data = vec![RequestType::Withdrawal as u8];
            for withdraw in &self.withdraws {
                data.extend(withdraw.encode()?);
            }
            res.push(data);
        }

        if !self.replace_by_fees.is_empty() {
            let mut data = vec![RequestType::ReplaceByFee as u8];
            for rbf in &self.replace_by_fees {
                data.extend(rbf.encode()?);
            }
            res.push(data);
        }

        if !self.cancel1s.is_empty() {
            let mut data = vec![RequestType::Cancel1 as u8];
            for cancel in &self.cancel1s {
                data.extend(cancel.encode()?);
            }
            res.push(data);
        }

        if !self.deposit_tax.is_empty() {
            let mut data = vec![RequestType::DepositTax as u8];
            for deposit in &self.deposit_tax {
                data.extend(deposit.encode()?);
            }
            res.push(data);
        }

        if !self.confirmation.is_empty() {
            let mut data = vec![RequestType::ConfirmationNumber as u8];
            for conf in &self.confirmation {
                data.extend(conf.encode()?);
            }
            res.push(data);
        }

        if !self.min_deposit.is_empty() {
            let mut data = vec![RequestType::MinDeposit as u8];
            for min_dep in &self.min_deposit {
                data.extend(min_dep.encode()?);
            }
            res.push(data);
        }

        Ok(res)
    }
}

#[derive(Debug, Clone, Default)]
pub struct WithdrawalRequest {
    pub id: u64,
    pub amount: u64,
    pub tx_price: u64,
    pub address: String,
}

pub fn unpack_into_withdraw_request(topics: &[B256], data: &[u8]) -> Result<WithdrawalRequest> {
    if topics.len() != 3 {
        bail!("Invalid Withdraw event topics length: expect 3 got {}", topics.len());
    }

    if data.len() < 192 || data.len() % 32 != 0 {
        bail!("Invalid Withdraw event data length: {}", data.len());
    }

    let id = U256::from_be_bytes(<[u8; 32]>::try_from(topics[1].as_slice())?);
    let id = id.try_into().context("Withdrawal ID is too large")?;

    let amount = U256::from_be_bytes(<[u8; 32]>::try_from(&data[0..32])?);
    let (amount, dust) = amount.div_rem(U256::from(SATOSHI));
    if !dust.is_zero() {
        bail!("Withdrawal amount has dust: {}", dust);
    }
    let amount = amount.try_into().context("Withdrawal amount is too large")?;

    let max_tx_price = U256::from_be_bytes(<[u8; 32]>::try_from(&data[64..96])?);
    let max_tx_price = max_tx_price.try_into().context("Max tx price is too large")?;

    let addr_loc = U256::from_be_bytes(<[u8; 32]>::try_from(&data[96..128])?);
    if addr_loc != U256::from(WITHDRAWAL_REQ_ADDR_LOC) {
        bail!("Address location in the withdraw event should be 128 but got {}", addr_loc);
    }

    let addr_len = U256::from_be_bytes(<[u8; 32]>::try_from(&data[128..160])?);
    let addr_len: usize = addr_len.try_into().context("Address length too large")?;
    if addr_len > 90 {
        bail!("Address length too large");
    }
    if data[160..].len() < addr_len {
        bail!("Address slice is out of range");
    }

    let address = String::from_utf8(data[160..160 + addr_len].to_vec())
        .context("Invalid UTF-8 in address")?;

    Ok(WithdrawalRequest { id, amount, tx_price: max_tx_price, address })
}

impl WithdrawalRequest {
    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut buf = Vec::new();

        buf.extend(self.id.to_le_bytes());
        buf.extend(self.amount.to_le_bytes());
        buf.extend(self.tx_price.to_le_bytes());

        if self.address.len() > 90 {
            bail!("address length exceeds maximum 90 bytes");
        }

        buf.push(self.address.len() as u8);
        buf.extend(self.address.as_bytes());

        Ok(buf)
    }
}

#[derive(Debug, Clone, Default)]
pub struct ReplaceByFeeRequest {
    pub id: u64,
    pub tx_price: u64,
}

pub fn unpack_into_replace_by_fee_request(
    topics: &[B256],
    data: &[u8],
) -> Result<ReplaceByFeeRequest> {
    if topics.len() != 2 {
        bail!("Invalid ReplaceByFee event topics length: expect 2 got {}", topics.len());
    }

    if data.len() != 32 {
        bail!("Invalid ReplaceByFee event data length: {}", data.len());
    }

    let id = U256::from_be_bytes(<[u8; 32]>::try_from(topics[1].as_slice())?);
    let id = id.try_into().context("Withdrawal ID is too large")?;

    let tx_price = U256::from_be_bytes(<[u8; 32]>::try_from(data)?);
    let tx_price = tx_price.try_into().context("Max tx price is too large")?;

    Ok(ReplaceByFeeRequest { id, tx_price })
}

impl ReplaceByFeeRequest {
    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut buf = Vec::new();

        buf.extend(self.id.to_le_bytes());
        buf.extend(self.tx_price.to_le_bytes());

        Ok(buf)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Cancel1Request {
    pub id: u64,
}

pub fn unpack_into_cancel1_request(topics: &[B256], data: &[u8]) -> Result<Cancel1Request> {
    if topics.len() != 2 {
        bail!("Invalid Cancel1 event topics length: expect 2 got {}", topics.len());
    }

    if !data.is_empty() {
        bail!("Invalid Cancel1 event data length, expect 0 got {}", data.len());
    }

    let id = U256::from_be_bytes(<[u8; 32]>::try_from(topics[1].as_slice())?);
    let id = id.try_into().context("Withdrawal ID is too large")?;

    Ok(Cancel1Request { id })
}

impl Cancel1Request {
    pub fn encode(&self) -> Result<Vec<u8>> {
        Ok(self.id.to_le_bytes().to_vec())
    }
}

#[derive(Debug, Clone, Default)]
pub struct DepositTaxRequest {
    pub rate: u64,
    pub max: u64,
}

pub fn unpack_into_deposit_tax_request(data: &[u8]) -> Result<DepositTaxRequest> {
    if data.len() != 64 {
        bail!("Invalid DepositTaxRequest event data length: {}", data.len());
    }

    let rate = U256::from_be_bytes(<[u8; 32]>::try_from(&data[..32])?);
    if rate > U256::from(MAX_TAX_RATE) {
        bail!("Deposit tax rate is too large");
    }
    let rate = rate.try_into().context("Failed to convert rate to u64")?;

    let max = U256::from_be_bytes(<[u8; 32]>::try_from(&data[32..])?);
    let (max, dust) = max.div_rem(U256::from(SATOSHI));

    if !dust.is_zero() {
        bail!("Max deposit tax has dust: {}", dust);
    }
    let max = max.try_into().context("Max deposit tax is too large")?;

    Ok(DepositTaxRequest { rate, max })
}

impl DepositTaxRequest {
    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut buf = Vec::new();

        buf.extend(self.rate.to_le_bytes());
        buf.extend(self.max.to_le_bytes());

        Ok(buf)
    }
}

#[derive(Debug, Clone, Default)]
pub struct ConfirmationNumberRequest {
    pub number: u64,
}

pub fn unpack_into_confirmation_number_request(data: &[u8]) -> Result<ConfirmationNumberRequest> {
    if data.len() != 32 {
        bail!("Invalid ConfirmationNumberRequest event data length: {}", data.len());
    }

    let number = U256::from_be_bytes(<[u8; 32]>::try_from(data)?);
    let number = number.try_into().context("Confirmation number is too large")?;

    Ok(ConfirmationNumberRequest { number })
}

impl ConfirmationNumberRequest {
    pub fn encode(&self) -> Result<Vec<u8>> {
        Ok(self.number.to_le_bytes().to_vec())
    }
}

#[derive(Debug, Clone, Default)]
pub struct MinDepositRequest {
    pub satoshi: u64,
}

pub fn unpack_into_min_deposit_request(data: &[u8]) -> Result<MinDepositRequest> {
    if data.len() != 32 {
        bail!("Invalid MinDepositRequest event data length: {}", data.len());
    }

    let amount = U256::from_be_bytes(<[u8; 32]>::try_from(data)?);
    let (amount, dust) = amount.div_rem(U256::from(SATOSHI));

    if !dust.is_zero() {
        bail!("Min deposit amount has dust: {}", dust);
    }

    let satoshi = amount.try_into().context("Min deposit value is too large")?;

    Ok(MinDepositRequest { satoshi })
}

impl MinDepositRequest {
    pub fn encode(&self) -> Result<Vec<u8>> {
        Ok(self.satoshi.to_le_bytes().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use alloy_primitives::{b256, hex};

    use super::*;

    #[test]
    fn test_unpack_into_withdraw_request() {
        // first test
        let topics = vec![
            b256!("be7c38d37e8132b1d2b29509df9bf58cf1126edf2563c00db0ef3a271fb9f35b"),
            b256!("0000000000000000000000000000000000000000000000000000000000000064"),
            b256!("0000000000000000000000005b38da6a701c568545dcfcb03fcb875f56beddc4"),
        ];
        let data = hex!("000000000000000000000000000000000000000000000000000000174876e800000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000003e62633171656e356b76336330657064397966717675327130353971736a7077753968646a797778327639703570396c386d73786e383866733979356b78360000");
        let req = unpack_into_withdraw_request(&topics, &data).unwrap();
        assert_eq!(req.id, 100);
        assert_eq!(req.amount, 10);
        assert_eq!(req.tx_price, 1);
        assert_eq!(req.address, "bc1qen5kv3c0epd9yfqvu2q059qsjpwu9hdjywx2v9p5p9l8msxn88fs9y5kx6");

        let encoded = hex!("64000000000000000a0000000000000001000000000000003e62633171656e356b76336330657064397966717675327130353971736a7077753968646a797778327639703570396c386d73786e383866733979356b7836");
        assert_eq!(req.encode().unwrap(), encoded);

        // second test
        let topics = vec![
            b256!("be7c38d37e8132b1d2b29509df9bf58cf1126edf2563c00db0ef3a271fb9f35b"),
            b256!("0000000000000000000000000000000000000000000000000000000000000001"),
            b256!("0000000000000000000000005b38da6a701c568545dcfcb03fcb875f56beddc4"),
        ];
        let data = hex!("0000000000000000000000000000000000000000000000000000002e90edd00000000000000000000000000000000000000000000000000000000000000003e8000000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000002a626331716d76733230387765336a67376867637a686c683765397566773033346b666d3276777376676500000000000000000000000000000000000000000000");
        let req = unpack_into_withdraw_request(&topics, &data).unwrap();
        assert_eq!(req.id, 1);
        assert_eq!(req.amount, 20);
        assert_eq!(req.tx_price, 10);
        assert_eq!(req.address, "bc1qmvs208we3jg7hgczhlh7e9ufw034kfm2vwsvge");

        let encoded = hex!("010000000000000014000000000000000a000000000000002a626331716d76733230387765336a67376867637a686c683765397566773033346b666d32767773766765");
        assert_eq!(req.encode().unwrap(), encoded);
    }

    #[test]
    fn test_unpack_into_replace_by_fee_request() {
        // first test
        let topics = vec![
            b256!("19875a7124af51c604454b74336ce2168c45bceade9d9a1e6dfae9ba7d31b7fa"),
            b256!("0000000000000000000000000000000000000000000000000000000000000001"),
        ];
        let data = hex!("0000000000000000000000000000000000000000000000000000000000000014");
        let req = unpack_into_replace_by_fee_request(&topics, &data).unwrap();
        assert_eq!(req.id, 1);
        assert_eq!(req.tx_price, 20);

        let encoded = hex!("01000000000000001400000000000000");
        assert_eq!(req.encode().unwrap(), encoded);

        // second test
        let topics = vec![
            b256!("19875a7124af51c604454b74336ce2168c45bceade9d9a1e6dfae9ba7d31b7fa"),
            b256!("0000000000000000000000000000000000000000000000000000000000000002"),
        ];
        let data = hex!("000000000000000000000000000000000000000000000000000000000000000a");
        let req = unpack_into_replace_by_fee_request(&topics, &data).unwrap();
        assert_eq!(req.id, 2);
        assert_eq!(req.tx_price, 10);

        let encoded = hex!("02000000000000000a00000000000000");
        assert_eq!(req.encode().unwrap(), encoded);
    }

    #[test]
    fn test_unpack_into_cancel1_request() {
        // first test
        let topics = vec![
            b256!("0106f4416537efff55311ef5e2f9c2a48204fcf84731f2b9d5091d23fc52160c"),
            b256!("0000000000000000000000000000000000000000000000000000000000000001"),
        ];
        let data = hex!("");
        let req = unpack_into_cancel1_request(&topics, &data).unwrap();
        assert_eq!(req.id, 1);

        let encoded = hex!("0100000000000000");
        assert_eq!(req.encode().unwrap(), encoded);

        // second test
        let topics = vec![
            b256!("0106f4416537efff55311ef5e2f9c2a48204fcf84731f2b9d5091d23fc52160c"),
            b256!("0000000000000000000000000000000000000000000000000000000000000002"),
        ];
        let data = hex!("");
        let req = unpack_into_cancel1_request(&topics, &data).unwrap();
        assert_eq!(req.id, 2);

        let encoded = hex!("0200000000000000");
        assert_eq!(req.encode().unwrap(), encoded);
    }

    #[test]
    fn test_unpack_into_deposit_tax_request() {
        let data = hex!("000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000005af3107a4000");
        let req = unpack_into_deposit_tax_request(&data).unwrap();
        assert_eq!(req.rate, 2);
        assert_eq!(req.max, 10000);

        let encoded = hex!("02000000000000001027000000000000");
        assert_eq!(req.encode().unwrap(), encoded);
    }

    #[test]
    fn test_unpack_into_confirmation_number_request() {
        let data = hex!("0000000000000000000000000000000000000000000000000000000000000006");
        let req = unpack_into_confirmation_number_request(&data).unwrap();
        assert_eq!(req.number, 6);

        let encoded = hex!("0600000000000000");
        assert_eq!(req.encode().unwrap(), encoded);
    }

    #[test]
    fn test_unpack_into_min_deposit_request() {
        let data = hex!("00000000000000000000000000000000000000000000000000005af3107a4000");
        let req = unpack_into_min_deposit_request(&data).unwrap();
        assert_eq!(req.satoshi, 10000);

        let encoded = hex!("1027000000000000");
        assert_eq!(req.encode().unwrap(), encoded);
    }
}
