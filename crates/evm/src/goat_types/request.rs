use alloy_primitives::{b256, B256};

pub const WITHDRAW_EVENT_TOPIC: B256 =
    b256!("be7c38d37e8132b1d2b29509df9bf58cf1126edf2563c00db0ef3a271fb9f35b");
pub const REPLACE_BY_FEE_EVENT_TOPIC: B256 =
    b256!("19875a7124af51c604454b74336ce2168c45bceade9d9a1e6dfae9ba7d31b7fa");
pub const CANCEL1_EVENT_TOPIC: B256 =
    b256!("0106f4416537efff55311ef5e2f9c2a48204fcf84731f2b9d5091d23fc52160c");
pub const CONFIRMATION_NUMBER_EVENT_TOPIC: B256 =
    b256!("30b92002139b64ec601b714d1ecccba1212034e735773b3de088e8876f4dfb65");
pub const UPDATE_MIN_DEPOSIT_EVENT_TOPIC: B256 =
    b256!("7aa20a242ea0b0f7b0141c56aaad636eb2b2077c9c27d09a8282f6931f486a21");
pub const UPDATE_DEPOSIT_TAX_EVENT_TOPIC: B256 =
    b256!("1007ff7aec53e9626ce51f25d4e093f290f60da8019c8cf489f0ae2f21ebf76a");

pub const ADD_VOTER_EVENT_TOPIC: B256 =
    b256!("101c617f43dd1b8a54a9d747d9121bbc55e93b88bc50560d782a79c4e28fc838");
pub const REMOVE_VOTER_EVENT_TOPIC: B256 =
    b256!("183393fc5cffbfc7d03d623966b85f76b9430f42d3aada2ac3f3deabc78899e8");

pub const CREATE_EVENT_TOPIC: B256 =
    b256!("f3aa84440b70359721372633122645674adb6dbb72622a222627248ef053a7dd");
pub const LOCK_EVENT_TOPIC: B256 =
    b256!("ec36c0364d931187a76cf66d7eee08fad0ec2e8b7458a8d8b26b36769d4d13f3");
pub const UNLOCK_EVENT_TOPIC: B256 =
    b256!("40f2a8c5e2e2a9ad2f4e4dfc69825595b526178445c3eb22b02edfd190601db7");
pub const CLAIM_EVENT_TOPIC: B256 =
    b256!("a983a6cfc4bd1095dac7b145ae020ba08e16cc7efa2051cc6b77e4011b9ee99b");
pub const GRANT_EVENT_TOPIC: B256 =
    b256!("41891e803e84c188180caa0f073ce4235b8002dac887a69fcdcae1d295951fa0");
pub const UPDATE_TOKEN_WEIGHT_EVENT_TOPIC: B256 =
    b256!("b59bf4596e5415117fb4625044cb5b0ca5b273742825b026d06afe82a48e6217");
pub const UPDATE_TOKEN_THRESHOLD_EVENT_TOPIC: B256 =
    b256!("326e29ab1c62c7d77fdfb302916e82e1a54f3b9961db75ee7e18afe488a0e92d");

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestType {
    Gas = 0,
    Create,
    Lock,
    Unlock,
    Claim,
    Grant,
    UpdateTokenWeight,
    UpdateTokenThreshold,
    Withdrawal = 11,
    ReplaceByFee,
    Cancel1,
    DepositTax,
    ConfirmationNumber,
    MinDeposit,
    AddVoter = 20,
    RemoveVoter,
}

#[cfg(test)]
mod tests {
    use alloc::{vec, vec::Vec};

    use alloy_primitives::{address, b256, b512, hex, U256};

    use crate::goat_types::*;

    #[test]
    fn test_encode_locking_requests() {
        let locking_req = LockingRequests {
            gas: vec![GasRequest::new(100, U256::from(1e9))],
            creates: vec![CreateRequest {
                validator: address!("94D76E24F818426ae84aa404140E8D5F60E10E7e"),
                pubkey: b512!("74602edafa25d1f5fcde1730328bf2c3559b47f689319ce478bd62f8ba582d35c6842e0533b4e5706d99e914b24da42dd35da09c0947223aa2ff88933c6bd946"),
            }, CreateRequest {
                validator: address!("35ed41deb0b9d86fdb8306c04927324a19f86509"),
                pubkey: b512!("8e2977a5198f9cd4b533fa71a7c06fe43dbdc52122f118475f42717f5e9166be0ee6d67614b93a89e00eb49c6c8dd9b24d5a39801c2d8a99b1aef5d5859fad62"),
            }],
            locks: vec![LockRequest {
                validator: address!("94D76E24F818426ae84aa404140E8D5F60E10E7e"),
                token: address!("0000000000000000000000000000000000000000"),
                amount: U256::from(10),
            }, LockRequest {
                validator: address!("35ed41deb0b9d86fdb8306c04927324a19f86509"),
                token: address!("9f42ff4c4ca8bfdcf6b9e15644a64eff6f226a2f"),
                amount: U256::from(2314),
            }, LockRequest {
                validator: address!("086f2b830d9952e065125b94bd86e1dcb00e24bc"),
                token: address!("b25fc880491c51cb7b1528c523a465d4ba23ca14"),
                amount: U256::from(1e18),
            }],
            unlocks: vec![UnlockRequest {
                id: 0,
                validator: address!("94D76E24F818426ae84aa404140E8D5F60E10E7e"),
                recipient: address!("5B38Da6a701c568545dCfcB03FcB875f56beddC4"),
                token: address!("0000000000000000000000000000000000000000"),
                amount: U256::from(10),
            }, UnlockRequest {
                id: 0,
                validator: address!("9563e3eb9ec48eceabbbfbb3f2a5a3c58ba471f1"),
                recipient: address!("a0a8930293dec1ea6b325966ccf5183b837e9d23"),
                token: address!("7bcc0bda4e9d8e4b011dcb0fb9846b67edd6f054"),
                amount: U256::from(10),
            }],
            claims: vec![ClaimRequest {
                id: 1,
                validator: address!("94D76E24F818426ae84aa404140E8D5F60E10E7e"),
                recipient: address!("5B38Da6a701c568545dCfcB03FcB875f56beddC4"),
            }, ClaimRequest {
                id: 2,
                validator: address!("94D76E24F818426ae84aa404140E8D5F60E10E7e"),
                recipient: address!("e1cc1fbe87cd99dfa40ea79916435563e12cc547"),
            }, ClaimRequest {
                id: 3,
                validator: address!("3a9f4a4c306ee85919b91559c749a8dcf40f978b"),
                recipient: address!("d4a520c05941d01df7b3e456658a7fc90ff0d378"),
            }],
            update_weights: vec![UpdateTokenWeightRequest {
                token: address!("0000000000000000000000000000000000000000"),
                weight: 10,
            }, UpdateTokenWeightRequest {
                token: address!("54c26679bc083c93ad58feb807bbd12c24a80c64"),
                weight: 10000,
            }, UpdateTokenWeightRequest {
                token: address!("0df08247173a183f20e4bdb254190091d26a909e"),
                weight: 100000,
            }],
            update_thresholds: vec![UpdateTokenThresholdRequest {
                token: address!("5B38Da6a701c568545dCfcB03FcB875f56beddC4"),
                threshold: U256::from(10),
            }, UpdateTokenThresholdRequest {
                token: address!("7d09bc661e0f8cb5e12d900f19654f7a92c7eb11"),
                threshold: U256::from(10),
            }],
            grants: vec![
                GrantRequest { amount: U256::from(0xcdd318c6 as u32) },
                GrantRequest { amount: U256::from(0x0e3e2dfb as u32) },
                GrantRequest { amount: U256::from(0x44ff0e05 as u32) },
            ],
        };

        let encoded: Vec<Vec<u8>> = vec![
            hex!("006400000000000000000000000000000000000000000000000000000000000000000000003b9aca00").into(),
            hex!("0194d76e24f818426ae84aa404140e8d5f60e10e7e74602edafa25d1f5fcde1730328bf2c3559b47f689319ce478bd62f8ba582d35c6842e0533b4e5706d99e914b24da42dd35da09c0947223aa2ff88933c6bd94635ed41deb0b9d86fdb8306c04927324a19f865098e2977a5198f9cd4b533fa71a7c06fe43dbdc52122f118475f42717f5e9166be0ee6d67614b93a89e00eb49c6c8dd9b24d5a39801c2d8a99b1aef5d5859fad62").into(),
            hex!("0294d76e24f818426ae84aa404140e8d5f60e10e7e0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000a35ed41deb0b9d86fdb8306c04927324a19f865099f42ff4c4ca8bfdcf6b9e15644a64eff6f226a2f000000000000000000000000000000000000000000000000000000000000090a086f2b830d9952e065125b94bd86e1dcb00e24bcb25fc880491c51cb7b1528c523a465d4ba23ca140000000000000000000000000000000000000000000000000de0b6b3a7640000").into(),
            hex!("03000000000000000094d76e24f818426ae84aa404140e8d5f60e10e7e5b38da6a701c568545dcfcb03fcb875f56beddc40000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000a00000000000000009563e3eb9ec48eceabbbfbb3f2a5a3c58ba471f1a0a8930293dec1ea6b325966ccf5183b837e9d237bcc0bda4e9d8e4b011dcb0fb9846b67edd6f054000000000000000000000000000000000000000000000000000000000000000a").into(),
            hex!("04010000000000000094d76e24f818426ae84aa404140e8d5f60e10e7e5b38da6a701c568545dcfcb03fcb875f56beddc4020000000000000094d76e24f818426ae84aa404140e8d5f60e10e7ee1cc1fbe87cd99dfa40ea79916435563e12cc54703000000000000003a9f4a4c306ee85919b91559c749a8dcf40f978bd4a520c05941d01df7b3e456658a7fc90ff0d378").into(),
            hex!("0500000000000000000000000000000000000000000000000000000000cdd318c6000000000000000000000000000000000000000000000000000000000e3e2dfb0000000000000000000000000000000000000000000000000000000044ff0e05").into(),
            hex!("0600000000000000000000000000000000000000000a0000000000000054c26679bc083c93ad58feb807bbd12c24a80c6410270000000000000df08247173a183f20e4bdb254190091d26a909ea086010000000000").into(),
            hex!("075b38da6a701c568545dcfcb03fcb875f56beddc4000000000000000000000000000000000000000000000000000000000000000a7d09bc661e0f8cb5e12d900f19654f7a92c7eb11000000000000000000000000000000000000000000000000000000000000000a").into(),
        ];

        for (a, b) in locking_req.encode().unwrap().iter().zip(encoded.iter()) {
            assert_eq!(a, b);
        }
    }

    #[test]
    fn test_encode_bridge_requests() {
        let bridge_req = BridgeRequests {
            withdraws: vec![
                WithdrawalRequest {
                    id: 1,
                    amount: 0xc60f7e8e,
                    tx_price: 1,
                    address: "1FwtBCsUqhQ8Zshw1or4aSUfdk1UDi83vh".to_string(),
                },
                WithdrawalRequest {
                    id: 2,
                    amount: 0xae212d1c,
                    tx_price: 2,
                    address: "bcrt1qxhk5rh4sh8vxlkurqmqyjfejfgvlsegfrd0yau".to_string(),
                },
                WithdrawalRequest {
                    id: 3,
                    amount: 0x5e357599,
                    tx_price: 2,
                    address: "bcrt1qxhk5rh4sh8vxlkurqmqyjfejfgvlsegfrd0yau".to_string(),
                },
                WithdrawalRequest {
                    id: 4,
                    amount: 0xbdebd2c6,
                    tx_price: 2,
                    address: "bc1qlkfkng9kcv8ancs8sycsfqm02fawqw24cf2am8u9hfl3vklvl2wsfujuzp"
                        .to_string(),
                },
                WithdrawalRequest {
                    id: 5,
                    amount: 0x8d7e9d08,
                    tx_price: 3,
                    address: "bc1qen5kv3c0epd9yfqvu2q059qsjpwu9hdjywx2v9p5p9l8msxn88fs9y5kx6"
                        .to_string(),
                },
                WithdrawalRequest {
                    id: 6,
                    amount: 0xbdebd2c6,
                    tx_price: 2,
                    address: "3Pbp8YCguJk9dXnTGqSXFnZbXC7EW8qbvy".to_string(),
                },
                WithdrawalRequest {
                    id: 7,
                    amount: 0x69b46a49,
                    tx_price: 2,
                    address: "17yhJ5DME9Fu3wVjVoVfP4jKxjrc9WRyaB".to_string(),
                },
                WithdrawalRequest {
                    id: 8,
                    amount: 0x5e357599,
                    tx_price: 2,
                    address: "2N1gW9FafSF8tRknY158GzXCTC5aygJEHgU".to_string(),
                },
            ],
            replace_by_fees: vec![
                ReplaceByFeeRequest { id: 1, tx_price: 10 },
                ReplaceByFeeRequest { id: 2, tx_price: 10 },
                ReplaceByFeeRequest { id: 3, tx_price: 10 },
            ],
            cancel1s: vec![
                Cancel1Request { id: 10 },
                Cancel1Request { id: 11 },
                Cancel1Request { id: 12 },
            ],
            deposit_tax: vec![DepositTaxRequest { rate: 2, max: 10000 }],
            confirmation: vec![ConfirmationNumberRequest { number: 6 }],
            min_deposit: vec![MinDepositRequest { satoshi: 10000 }],
        };

        let encoded: Vec<Vec<u8>> = vec![
            hex!("0b01000000000000008e7e0fc6000000000100000000000000223146777442437355716851385a736877316f723461535566646b315544693833766802000000000000001c2d21ae0000000002000000000000002c62637274317178686b3572683473683876786c6b7572716d71796a66656a6667766c7365676672643079617503000000000000009975355e0000000002000000000000002c62637274317178686b3572683473683876786c6b7572716d71796a66656a6667766c736567667264307961750400000000000000c6d2ebbd0000000002000000000000003e626331716c6b666b6e67396b637638616e6373387379637366716d303266617771773234636632616d38753968666c33766b6c766c32777366756a757a700500000000000000089d7e8d0000000003000000000000003e62633171656e356b76336330657064397966717675327130353971736a7077753968646a797778327639703570396c386d73786e383866733979356b78360600000000000000c6d2ebbd000000000200000000000000223350627038594367754a6b3964586e5447715358466e5a62584337455738716276790700000000000000496ab46900000000020000000000000022313779684a35444d453946753377566a566f566650346a4b786a726339575279614208000000000000009975355e00000000020000000000000023324e3167573946616653463874526b6e59313538477a58435443356179674a45486755").into(),
            hex!("0c01000000000000000a0000000000000002000000000000000a0000000000000003000000000000000a00000000000000").into(),
            hex!("0d0a000000000000000b000000000000000c00000000000000").into(),
            hex!("0e02000000000000001027000000000000").into(),
            hex!("0f0600000000000000").into(),
            hex!("101027000000000000").into(),
        ];

        for (a, b) in bridge_req.encode().unwrap().iter().zip(encoded.iter()) {
            assert_eq!(a, b);
        }
    }

    #[test]
    fn test_encode_relayer_requests() {
        let relayer_req = RelayerRequests {
            adds: vec![
                AddVoterRequest {
                    voter: address!("5B38Da6a701c568545dCfcB03FcB875f56beddC4"),
                    pubkey: b256!(
                        "13e21ffd05c7c3e6e7695535f91de949a7a31c577930b537482e1f192b5af389"
                    ),
                },
                AddVoterRequest {
                    voter: address!("dffa2082a93b579528ef045adda07076a649eb57"),
                    pubkey: b256!(
                        "2c8b82bd642ad4ec4e393c1e077573a3cd7d665906de9fae1833a5b38117e659"
                    ),
                },
                AddVoterRequest {
                    voter: address!("e21c34698b4c784d1775b4e9255ca6ffdccbb95a"),
                    pubkey: b256!(
                        "9e75b1ec21ab8590ede512c126552756a3c4e94d45fd05b31362c17e922b346f"
                    ),
                },
            ],
            removes: vec![
                RemoveVoterRequest { voter: address!("5B38Da6a701c568545dCfcB03FcB875f56beddC4") },
                RemoveVoterRequest { voter: address!("6a20b59fb205df2004deef70e4020bbf2f91ca98") },
                RemoveVoterRequest { voter: address!("b8299a5697f56266186ccdd8d2ef4a2df76f1857") },
            ],
        };

        let encoded: Vec<Vec<u8>> = vec![
            hex!("145b38da6a701c568545dcfcb03fcb875f56beddc413e21ffd05c7c3e6e7695535f91de949a7a31c577930b537482e1f192b5af389dffa2082a93b579528ef045adda07076a649eb572c8b82bd642ad4ec4e393c1e077573a3cd7d665906de9fae1833a5b38117e659e21c34698b4c784d1775b4e9255ca6ffdccbb95a9e75b1ec21ab8590ede512c126552756a3c4e94d45fd05b31362c17e922b346f").into(),
            hex!("155b38da6a701c568545dcfcb03fcb875f56beddc46a20b59fb205df2004deef70e4020bbf2f91ca98b8299a5697f56266186ccdd8d2ef4a2df76f1857").into(),
        ];

        for (a, b) in relayer_req.encode().unwrap().iter().zip(encoded.iter()) {
            assert_eq!(a, b);
        }
    }
}
