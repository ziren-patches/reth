use alloc::{vec, vec::Vec};

use alloy_primitives::{Address, B512, U256};
use anyhow::{bail, Context, Result};

use super::request::*;

#[derive(Debug, Clone, Default)]
pub struct LockingRequests {
    pub gas: Vec<GasRequest>,
    pub creates: Vec<CreateRequest>,
    pub locks: Vec<LockRequest>,
    pub unlocks: Vec<UnlockRequest>,
    pub claims: Vec<ClaimRequest>,
    pub grants: Vec<GrantRequest>,
    pub update_weights: Vec<UpdateTokenWeightRequest>,
    pub update_thresholds: Vec<UpdateTokenThresholdRequest>,
}

impl LockingRequests {
    pub fn encode(&self) -> Result<Vec<Vec<u8>>> {
        let mut res = Vec::new();

        if !self.gas.is_empty() {
            let mut gas_data = vec![RequestType::Gas as u8];
            for req in &self.gas {
                gas_data.extend(req.encode().context("Failed to encode GasRequest")?);
            }
            res.push(gas_data);
        }

        if !self.creates.is_empty() {
            let mut creates_data = vec![RequestType::Create as u8];
            for req in &self.creates {
                creates_data.extend(req.encode().context("Failed to encode CreateRequest")?);
            }
            res.push(creates_data);
        }

        if !self.locks.is_empty() {
            let mut locks_data = vec![RequestType::Lock as u8];
            for req in &self.locks {
                locks_data.extend(req.encode().context("Failed to encode LockRequest")?);
            }
            res.push(locks_data);
        }

        if !self.unlocks.is_empty() {
            let mut unlocks_data = vec![RequestType::Unlock as u8];
            for req in &self.unlocks {
                unlocks_data.extend(req.encode().context("Failed to encode UnlockRequest")?);
            }
            res.push(unlocks_data);
        }

        if !self.claims.is_empty() {
            let mut claims_data = vec![RequestType::Claim as u8];
            for req in &self.claims {
                claims_data.extend(req.encode().context("Failed to encode ClaimRequest")?);
            }
            res.push(claims_data);
        }

        if !self.grants.is_empty() {
            let mut grants_data = vec![RequestType::Grant as u8];
            for req in &self.grants {
                grants_data.extend(req.encode().context("Failed to encode GrantRequest")?);
            }
            res.push(grants_data);
        }

        if !self.update_weights.is_empty() {
            let mut weights_data = vec![RequestType::UpdateTokenWeight as u8];
            for req in &self.update_weights {
                weights_data
                    .extend(req.encode().context("Failed to encode UpdateTokenWeightRequest")?);
            }
            res.push(weights_data);
        }

        if !self.update_thresholds.is_empty() {
            let mut thresholds_data = vec![RequestType::UpdateTokenThreshold as u8];
            for req in &self.update_thresholds {
                thresholds_data
                    .extend(req.encode().context("Failed to encode UpdateTokenThresholdRequest")?);
            }
            res.push(thresholds_data);
        }

        Ok(res)
    }
}

#[derive(Debug, Clone, Default)]
pub struct GasRequest {
    pub height: u64,
    pub amount: U256,
}

impl GasRequest {
    pub fn new(height: u64, amount: U256) -> Self {
        Self { height, amount }
    }

    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut res = Vec::with_capacity(40);

        res.extend(&self.height.to_le_bytes());
        res.extend(&self.amount.to_be_bytes::<32>());

        Ok(res)
    }
}

#[derive(Debug, Clone, Default)]
pub struct CreateRequest {
    pub validator: Address,
    pub pubkey: B512,
}

pub fn unpack_into_create_request(data: &[u8]) -> Result<CreateRequest> {
    if data.len() != 128 {
        bail!("Invalid CreateValidator event data length: want 128, have {}", data.len());
    }

    let validator = Address::from_slice(&data[12..32]);
    let pubkey = B512::from_slice(&data[64..128]);

    Ok(CreateRequest { validator, pubkey })
}

impl CreateRequest {
    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut buf = Vec::new();

        buf.extend(&self.validator);
        buf.extend(&self.pubkey);

        Ok(buf)
    }
}

#[derive(Debug, Clone, Default)]
pub struct LockRequest {
    pub validator: Address,
    pub token: Address,
    pub amount: U256,
}

pub fn unpack_into_lock_request(data: &[u8]) -> Result<LockRequest> {
    if data.len() != 96 {
        bail!("Invalid Lock event data length: want 96, have {}", data.len());
    }

    let validator = Address::from_slice(&data[12..32]);

    let token = Address::from_slice(&data[44..64]);

    let amount = U256::from_be_bytes(<[u8; 32]>::try_from(&data[64..])?);

    Ok(LockRequest { validator, token, amount })
}

impl LockRequest {
    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut res = Vec::with_capacity(72);

        if self.validator.len() != 20 {
            bail!("Validator must be 20 bytes");
        }
        res.extend(&self.validator);

        if self.token.len() != 20 {
            bail!("Token must be 20 bytes");
        }
        res.extend(&self.token);

        res.extend(self.amount.to_be_bytes::<32>());
        Ok(res)
    }
}

#[derive(Debug, Clone, Default)]
pub struct UnlockRequest {
    pub id: u64,
    pub validator: Address,
    pub recipient: Address,
    pub token: Address,
    pub amount: U256,
}

pub fn unpack_into_unlock_request(data: &[u8]) -> Result<UnlockRequest> {
    if data.len() != 160 {
        bail!("Invalid Unlock event data length: want 160, have {}", data.len());
    }

    let id = U256::from_be_bytes(<[u8; 32]>::try_from(&data[..32])?)
        .try_into()
        .context("ID value is too large")?;

    let validator = Address::from_slice(&data[44..64]);

    let recipient = Address::from_slice(&data[76..96]);

    let token = Address::from_slice(&data[108..128]);

    let amount = U256::from_be_bytes(<[u8; 32]>::try_from(&data[128..160])?);

    Ok(UnlockRequest { id, validator, recipient, token, amount })
}

impl UnlockRequest {
    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut res = Vec::with_capacity(100);

        res.extend(&self.id.to_le_bytes());

        if self.validator.len() != 20 {
            bail!("Validator must be 20 bytes");
        }
        res.extend(&self.validator);

        if self.recipient.len() != 20 {
            bail!("Recipient must be 20 bytes");
        }
        res.extend(&self.recipient);

        if self.token.len() != 20 {
            bail!("Token must be 20 bytes");
        }
        res.extend(&self.token);

        res.extend(self.amount.to_be_bytes::<32>());
        Ok(res)
    }
}

#[derive(Debug, Clone, Default)]
pub struct ClaimRequest {
    pub id: u64,
    pub validator: Address,
    pub recipient: Address,
}

pub fn unpack_into_claim_request(data: &[u8]) -> Result<ClaimRequest> {
    if data.len() != 96 {
        bail!("GoatRewardClaim wrong length: want 96, have {}", data.len());
    }

    let id = U256::from_be_bytes(<[u8; 32]>::try_from(&data[..32])?)
        .try_into()
        .context("Claim ID value is too large")?;

    let validator = Address::from_slice(&data[44..64]);
    let recipient = Address::from_slice(&data[76..96]);

    Ok(ClaimRequest { id, validator, recipient })
}

impl ClaimRequest {
    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut res = Vec::with_capacity(48);

        res.extend(&self.id.to_le_bytes());

        if self.validator.len() != 20 {
            bail!("Validator must be 20 bytes");
        }
        res.extend(&self.validator);

        if self.recipient.len() != 20 {
            bail!("Recipient must be 20 bytes");
        }
        res.extend(&self.recipient);

        Ok(res)
    }
}

#[derive(Debug, Clone, Default)]
pub struct UpdateTokenWeightRequest {
    pub token: Address,
    pub weight: u64,
}

pub fn unpack_into_update_token_weight_request(data: &[u8]) -> Result<UpdateTokenWeightRequest> {
    if data.len() != 64 {
        bail!("UpdateTokenWeight wrong length: want 64, have {}", data.len());
    }

    let token = Address::from_slice(&data[12..32]);

    let weight = U256::from_be_bytes(<[u8; 32]>::try_from(&data[32..64])?)
        .try_into()
        .context("Weight value is too large")?;

    Ok(UpdateTokenWeightRequest { token, weight })
}

impl UpdateTokenWeightRequest {
    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut res = Vec::with_capacity(28);

        if self.token.len() != 20 {
            bail!("Token must be 20 bytes");
        }
        res.extend(&self.token);

        res.extend(&self.weight.to_le_bytes());

        Ok(res)
    }
}

#[derive(Debug, Clone, Default)]
pub struct UpdateTokenThresholdRequest {
    pub token: Address,
    pub threshold: U256,
}

pub fn unpack_into_update_token_threshold_request(
    data: &[u8],
) -> Result<UpdateTokenThresholdRequest> {
    if data.len() != 64 {
        bail!("Invalid UpdateTokenThreshold event data length: want 64, have {}", data.len());
    }

    let token = Address::from_slice(&data[12..32]);
    let threshold = U256::from_be_bytes(<[u8; 32]>::try_from(&data[32..64])?);

    Ok(UpdateTokenThresholdRequest { token, threshold })
}

impl UpdateTokenThresholdRequest {
    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut res = Vec::with_capacity(52);

        if self.token.len() != 20 {
            bail!("Token must be 20 bytes");
        }
        res.extend(&self.token);

        res.extend(self.threshold.to_be_bytes::<32>());
        Ok(res)
    }
}

#[derive(Debug, Clone, Default)]
pub struct GrantRequest {
    pub amount: U256,
}

pub fn unpack_into_grant_request(data: &[u8]) -> Result<GrantRequest> {
    if data.len() != 32 {
        bail!("Invalid GoatGrant event data length: want 32, have {}", data.len());
    }

    let amount = U256::from_be_bytes(<[u8; 32]>::try_from(data)?);

    Ok(GrantRequest { amount })
}

impl GrantRequest {
    pub fn encode(&self) -> Result<Vec<u8>> {
        Ok(self.amount.to_be_bytes::<32>().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use alloy_primitives::{address, b512, hex};

    use super::*;

    #[test]
    fn test_new_gas_request() {
        let req = GasRequest::new(1, U256::from(2));
        let encoded = hex!(
            "01000000000000000000000000000000000000000000000000000000000000000000000000000002"
        );
        assert_eq!(req.encode().unwrap(), encoded);
    }

    #[test]
    fn test_unpack_into_create_request() {
        let data = hex!("0000000000000000000000008945a1288dc78a6d8952a92c77aee6730b4147780000000000000000000000005b38da6a701c568545dcfcb03fcb875f56beddc4b21124a8e21a475a08e4bf1ad6940f52b105d065075f610227089981948d81b0df0b6e43fc4c228a48ff159c3e6a38eb0e6ce15d78312a445d3d1671fe756842");
        let req = unpack_into_create_request(&data).unwrap();
        assert_eq!(req.validator, address!("8945a1288dc78a6d8952a92c77aee6730b414778"));
        assert_eq!(req.pubkey, b512!("b21124a8e21a475a08e4bf1ad6940f52b105d065075f610227089981948d81b0df0b6e43fc4c228a48ff159c3e6a38eb0e6ce15d78312a445d3d1671fe756842"));

        let encoded = hex!("8945a1288dc78a6d8952a92c77aee6730b414778b21124a8e21a475a08e4bf1ad6940f52b105d065075f610227089981948d81b0df0b6e43fc4c228a48ff159c3e6a38eb0e6ce15d78312a445d3d1671fe756842");
        assert_eq!(req.encode().unwrap(), encoded);
    }

    #[test]
    fn test_unpack_into_lock_request() {
        let data = hex!("0000000000000000000000008945a1288dc78a6d8952a92c77aee6730b4147780000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000a");
        let req = unpack_into_lock_request(&data).unwrap();
        assert_eq!(req.validator, address!("8945A1288dc78A6D8952a92C77aEe6730B414778"));
        assert_eq!(req.token, address!("0000000000000000000000000000000000000000"));
        assert_eq!(req.amount, U256::from(10));

        let encoded = hex!("8945a1288dc78a6d8952a92c77aee6730b4147780000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000a");
        assert_eq!(req.encode().unwrap(), encoded);
    }

    #[test]
    fn test_unpack_into_unlock_request() {
        let data = hex!("00000000000000000000000000000000000000000000000000000000000000000000000000000000000000008945a1288dc78a6d8952a92c77aee6730b4147780000000000000000000000005b38da6a701c568545dcfcb03fcb875f56beddc40000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000a");
        let req = unpack_into_unlock_request(&data).unwrap();
        assert_eq!(req.validator, address!("8945A1288dc78A6D8952a92C77aEe6730B414778"));
        assert_eq!(req.recipient, address!("5B38Da6a701c568545dCfcB03FcB875f56beddC4"));
        assert_eq!(req.token, address!("0000000000000000000000000000000000000000"));
        assert_eq!(req.amount, U256::from(10));

        let encoded = hex!("00000000000000008945a1288dc78a6d8952a92c77aee6730b4147785b38da6a701c568545dcfcb03fcb875f56beddc40000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000a");
        assert_eq!(req.encode().unwrap(), encoded);
    }

    #[test]
    fn test_unpack_into_claim_request() {
        let data = hex!("00000000000000000000000000000000000000000000000000000000000000010000000000000000000000008945a1288dc78a6d8952a92c77aee6730b4147780000000000000000000000005b38da6a701c568545dcfcb03fcb875f56beddc4");
        let req = unpack_into_claim_request(&data).unwrap();
        assert_eq!(req.id, 1);
        assert_eq!(req.validator, address!("8945A1288dc78A6D8952a92C77aEe6730B414778"));
        assert_eq!(req.recipient, address!("5B38Da6a701c568545dCfcB03FcB875f56beddC4"));

        let encoded = hex!("01000000000000008945a1288dc78a6d8952a92c77aee6730b4147785b38da6a701c568545dcfcb03fcb875f56beddc4");
        assert_eq!(req.encode().unwrap(), encoded);
    }

    #[test]
    fn test_unpack_into_update_token_weight_request() {
        let data = hex!("0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000a");
        let req = unpack_into_update_token_weight_request(&data).unwrap();
        assert_eq!(req.token, address!("0000000000000000000000000000000000000000"));
        assert_eq!(req.weight, 10);

        let encoded = hex!("00000000000000000000000000000000000000000a00000000000000");
        assert_eq!(req.encode().unwrap(), encoded);
    }

    #[test]
    fn test_unpack_into_update_token_threshold_request() {
        let data = hex!("0000000000000000000000005b38da6a701c568545dcfcb03fcb875f56beddc4000000000000000000000000000000000000000000000000000000000000000a");
        let req = unpack_into_update_token_threshold_request(&data).unwrap();
        assert_eq!(req.token, address!("5B38Da6a701c568545dCfcB03FcB875f56beddC4"));
        assert_eq!(req.threshold, U256::from(10));

        let encoded = hex!("5b38da6a701c568545dcfcb03fcb875f56beddc4000000000000000000000000000000000000000000000000000000000000000a");
        assert_eq!(req.encode().unwrap(), encoded);
    }

    #[test]
    fn test_unpack_into_grant_request() {
        let data = hex!("000000000000000000000000000000000000000000000000000000000000000a");
        let req = unpack_into_grant_request(&data).unwrap();
        assert_eq!(req.amount, U256::from(10));

        let encoded = hex!("000000000000000000000000000000000000000000000000000000000000000a");
        assert_eq!(req.encode().unwrap(), encoded);
    }
}
