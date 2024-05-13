pub mod contract;

use contract::ERC20Token;
use revm::{
    db::PlainAccount,
    primitives::{uint, AccountInfo, Address, TransactTo, TxEnv, U256},
};

fn generate_addresses(length: usize) -> Vec<Address> {
    (0..length).map(|_| Address::new(rand::random())).collect()
}

pub fn generate_clusters(
    num_clusters: usize,
    num_people_per_cluster: usize,
    num_transfers_per_person: usize,
) -> (Vec<(Address, PlainAccount)>, Vec<TxEnv>) {
    let clusters: Vec<Vec<Address>> = (0..num_clusters)
        .map(|_| generate_addresses(num_people_per_cluster))
        .collect();

    let people_addresses: Vec<Address> = clusters.clone().into_iter().flatten().collect();

    let gld_address = Address::new(rand::random());

    let gld_account = ERC20Token::new("Gold Token", "GLD", 18, 222_222_000_000_000_000_000_000u128)
        .add_balances(&people_addresses, uint!(1_000_000_000_000_000_000_U256))
        .build();

    let mut state = Vec::from(&[(gld_address, gld_account)]);
    let mut txs = Vec::new();

    for person in people_addresses.iter() {
        let info = AccountInfo::from_balance(uint!(4_567_000_000_000_000_000_000_U256));
        state.push((*person, PlainAccount::from(info)));
    }

    for nonce in 0..num_transfers_per_person {
        for cluster in clusters.iter() {
            for person in cluster {
                let recipient = cluster[(rand::random::<usize>()) % (cluster.len())];
                let calldata = ERC20Token::transfer(recipient, U256::from(rand::random::<u8>()));

                txs.push(TxEnv {
                    caller: *person,
                    gas_limit: 16_777_216u64,
                    gas_price: U256::from(0xb2d05e07u64),
                    transact_to: TransactTo::Call(gld_address),
                    value: U256::ZERO,
                    data: calldata,
                    nonce: Some(nonce as u64),
                    ..TxEnv::default()
                })
            }
        }
    }

    (state, txs)
}
