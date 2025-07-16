use alloc::{vec, vec::Vec};

use alloy_primitives::{Address, B256};
use anyhow::{bail, Result};

use super::request::*;

#[derive(Debug, Clone, Default)]
pub struct RelayerRequests {
    pub adds: Vec<AddVoterRequest>,
    pub removes: Vec<RemoveVoterRequest>,
}

impl RelayerRequests {
    pub fn encode(&self) -> Result<Vec<Vec<u8>>> {
        let mut res = Vec::new();

        if !self.adds.is_empty() {
            let mut adds_bytes = vec![RequestType::AddVoter as u8];
            for req in &self.adds {
                adds_bytes.extend(req.encode()?);
            }
            res.push(adds_bytes);
        }

        if !self.removes.is_empty() {
            let mut removes_bytes = vec![RequestType::RemoveVoter as u8];
            for req in &self.removes {
                removes_bytes.extend(req.encode()?);
            }
            res.push(removes_bytes);
        }

        Ok(res)
    }
}

#[derive(Debug, Clone, Default)]
pub struct AddVoterRequest {
    pub voter: Address,
    pub pubkey: B256,
}

pub fn unpack_into_add_voter_request(topics: &[B256], data: &[u8]) -> Result<AddVoterRequest> {
    if topics.len() != 2 {
        bail!("Invalid AddVoter event topics length: expect 2 got {}", topics.len());
    }

    if data.len() != 32 {
        bail!("Invalid AddVoter event data length: want 32 have {}", data.len());
    }

    let voter = Address::from_slice(&topics[1].as_slice()[topics[1].len() - 20..]);

    let pubkey = B256::from_slice(data);

    Ok(AddVoterRequest { voter, pubkey })
}

impl AddVoterRequest {
    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut res = Vec::with_capacity(52);

        if self.voter.len() != 20 {
            bail!("Voter must be 20 bytes");
        }
        res.extend(&self.voter);

        if self.pubkey.len() != 32 {
            bail!("Pubkey must be 32 bytes");
        }
        res.extend(&self.pubkey);

        Ok(res)
    }
}

#[derive(Debug, Clone, Default)]
pub struct RemoveVoterRequest {
    pub voter: Address,
}

pub fn unpack_into_remove_voter_request(
    topics: &[B256],
    data: &[u8],
) -> Result<RemoveVoterRequest> {
    if topics.len() != 2 {
        bail!("Invalid RemoveVoter event topics length: expect 2 got {}", topics.len());
    }

    if !data.is_empty() {
        bail!("Invalid RemoveVoter event data length: want 0, have {}", data.len());
    }

    let voter = Address::from_slice(&topics[1].as_slice()[topics[1].len() - 20..]);

    Ok(RemoveVoterRequest { voter })
}

impl RemoveVoterRequest {
    pub fn encode(&self) -> Result<Vec<u8>> {
        Ok(self.voter.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use alloy_primitives::{address, b256, hex};

    use super::*;

    #[test]
    fn test_unpack_into_add_voter_request() {
        let topics = vec![
            b256!("101c617f43dd1b8a54a9d747d9121bbc55e93b88bc50560d782a79c4e28fc838"),
            b256!("000000000000000000000000d12a5a92d4621fbe3068914988d538c410245443"),
        ];
        let data = hex!("023504e3cadac49656b8f0ac939b1665870c5eb60cd47541e401babb7ff99f23");
        let req = unpack_into_add_voter_request(&topics, &data).unwrap();
        assert_eq!(req.voter, address!("d12a5a92D4621fBE3068914988D538c410245443"));
        assert_eq!(
            req.pubkey,
            b256!("023504e3cadac49656b8f0ac939b1665870c5eb60cd47541e401babb7ff99f23")
        );
    }

    #[test]
    fn test_unpack_into_remove_voter_request() {
        let topics = vec![
            b256!("183393fc5cffbfc7d03d623966b85f76b9430f42d3aada2ac3f3deabc78899e8"),
            b256!("000000000000000000000000c96397756df86d3ac4c04958ee5bf9ac7421e328"),
        ];
        let data = hex!("");
        let req = unpack_into_remove_voter_request(&topics, &data).unwrap();
        assert_eq!(req.voter, address!("c96397756df86d3ac4c04958ee5bf9ac7421e328"));
    }
}
