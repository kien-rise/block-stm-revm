// Test ERC20 transfers.
// Currently, we only have a no-state-conflict test of user accounts sending to themselves.
// TODO: Add more tests of accounts cross-transferring to create state depdendencies.

use revm::primitives::{
    address, bytes, fixed_bytes, keccak256, uint, Account, AccountInfo, Address, BlockEnv,
    Bytecode, Bytes, SpecId, StorageSlot, TransactTo, TxEnv, B256, U256,
};

pub mod common;

// Mock an ERC20 account and give each passed in user account a token
fn mock_erc20_account(user_addresses: &[Address]) -> (Address, Account) {
    let address = address!("fbfbfddd6e35da57b7b0f9a2c10e34be70b3a4e9");
    let bytecode = Bytecode::new_raw(bytes!("608060405234801561001057600080fd5b50600436106100a95760003560e01c8063395093511161007157806339509351146101d957806370a082311461020557806395d89b411461022b578063a457c2d714610233578063a9059cbb1461025f578063dd62ed3e1461028b576100a9565b806306fdde03146100ae578063095ea7b31461012b57806318160ddd1461016b57806323b872dd14610185578063313ce567146101bb575b600080fd5b6100b66102b9565b6040805160208082528351818301528351919283929083019185019080838360005b838110156100f05781810151838201526020016100d8565b50505050905090810190601f16801561011d5780820380516001836020036101000a031916815260200191505b509250505060405180910390f35b6101576004803603604081101561014157600080fd5b506001600160a01b03813516906020013561034f565b604080519115158252519081900360200190f35b61017361036c565b60408051918252519081900360200190f35b6101576004803603606081101561019b57600080fd5b506001600160a01b03813581169160208101359091169060400135610372565b6101c36103f9565b6040805160ff9092168252519081900360200190f35b610157600480360360408110156101ef57600080fd5b506001600160a01b038135169060200135610402565b6101736004803603602081101561021b57600080fd5b50356001600160a01b0316610450565b6100b661046b565b6101576004803603604081101561024957600080fd5b506001600160a01b0381351690602001356104cc565b6101576004803603604081101561027557600080fd5b506001600160a01b038135169060200135610534565b610173600480360360408110156102a157600080fd5b506001600160a01b0381358116916020013516610548565b60038054604080516020601f60026000196101006001881615020190951694909404938401819004810282018101909252828152606093909290918301828280156103455780601f1061031a57610100808354040283529160200191610345565b820191906000526020600020905b81548152906001019060200180831161032857829003601f168201915b5050505050905090565b600061036361035c6105d4565b84846105d8565b50600192915050565b60025490565b600061037f8484846106c4565b6103ef8461038b6105d4565b6103ea85604051806060016040528060288152602001610927602891396001600160a01b038a166000908152600160205260408120906103c96105d4565b6001600160a01b03168152602081019190915260400160002054919061081f565b6105d8565b5060019392505050565b60055460ff1690565b600061036361040f6105d4565b846103ea85600160006104206105d4565b6001600160a01b03908116825260208083019390935260409182016000908120918c168152925290205490610573565b6001600160a01b031660009081526020819052604090205490565b60048054604080516020601f60026000196101006001881615020190951694909404938401819004810282018101909252828152606093909290918301828280156103455780601f1061031a57610100808354040283529160200191610345565b60006103636104d96105d4565b846103ea8560405180606001604052806025815260200161099860259139600160006105036105d4565b6001600160a01b03908116825260208083019390935260409182016000908120918d1681529252902054919061081f565b60006103636105416105d4565b84846106c4565b6001600160a01b03918216600090815260016020908152604080832093909416825291909152205490565b6000828201838110156105cd576040805162461bcd60e51b815260206004820152601b60248201527f536166654d6174683a206164646974696f6e206f766572666c6f770000000000604482015290519081900360640190fd5b9392505050565b3390565b6001600160a01b03831661061d5760405162461bcd60e51b81526004018080602001828103825260248152602001806109746024913960400191505060405180910390fd5b6001600160a01b0382166106625760405162461bcd60e51b81526004018080602001828103825260228152602001806108df6022913960400191505060405180910390fd5b6001600160a01b03808416600081815260016020908152604080832094871680845294825291829020859055815185815291517f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b9259281900390910190a3505050565b6001600160a01b0383166107095760405162461bcd60e51b815260040180806020018281038252602581526020018061094f6025913960400191505060405180910390fd5b6001600160a01b03821661074e5760405162461bcd60e51b81526004018080602001828103825260238152602001806108bc6023913960400191505060405180910390fd5b6107598383836108b6565b61079681604051806060016040528060268152602001610901602691396001600160a01b038616600090815260208190526040902054919061081f565b6001600160a01b0380851660009081526020819052604080822093909355908416815220546107c59082610573565b6001600160a01b038084166000818152602081815260409182902094909455805185815290519193928716927fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef92918290030190a3505050565b600081848411156108ae5760405162461bcd60e51b81526004018080602001828103825283818151815260200191508051906020019080838360005b8381101561087357818101518382015260200161085b565b50505050905090810190601f1680156108a05780820380516001836020036101000a031916815260200191505b509250505060405180910390fd5b505050900390565b50505056fe45524332303a207472616e7366657220746f20746865207a65726f206164647265737345524332303a20617070726f766520746f20746865207a65726f206164647265737345524332303a207472616e7366657220616d6f756e7420657863656564732062616c616e636545524332303a207472616e7366657220616d6f756e74206578636565647320616c6c6f77616e636545524332303a207472616e736665722066726f6d20746865207a65726f206164647265737345524332303a20617070726f76652066726f6d20746865207a65726f206164647265737345524332303a2064656372656173656420616c6c6f77616e63652062656c6f77207a65726fa2646970667358221220af410612545251aac98e209cf5be29983b4a961bded32de26d3322fdc6305ef864736f6c63430007060033"));

    let mut account = Account::from(AccountInfo::new(
        U256::ZERO,
        1u64,
        bytecode.hash_slow(),
        bytecode,
    ));

    // _balances: mapping(address => uint256)
    account.storage.insert(
        U256::from(0),
        StorageSlot::new(uint!(
            0x0000000000000000000000000000000000000000000000000000000000000000_U256
        )),
    );

    // _allowances: mapping(address => mapping(address => uint256))
    account.storage.insert(
        U256::from(1),
        StorageSlot::new(uint!(
            0x0000000000000000000000000000000000000000000000000000000000000000_U256
        )),
    );

    // _totalSupply: uint256
    account.storage.insert(
        U256::from(2),
        StorageSlot::new(uint!(
            0x00000000000000000000000000000000000000000001056e02dc4bb2ddbc0000_U256
        )),
    );

    // _name: string
    account.storage.insert(
        U256::from(3),
        StorageSlot::new(uint!(
            0x476f6c6420546f6b656e00000000000000000000000000000000000000000014_U256
        )),
    );

    // _symbol: string
    account.storage.insert(
        U256::from(4),
        StorageSlot::new(uint!(
            0x474c440000000000000000000000000000000000000000000000000000000006_U256
        )),
    );

    // _decimals: uint8
    account.storage.insert(
        U256::from(5),
        StorageSlot::new(uint!(
            0x0000000000000000000000000000000000000000000000000000000000000012_U256
        )),
    );

    // Give each user address a token
    for user_address in user_addresses.iter() {
        let storage_key =
            keccak256([B256::left_padding_from(user_address.as_slice()), B256::ZERO].concat());
        account
            .storage
            .insert(storage_key.into(), StorageSlot::new(U256::from(1)));
    }

    (address, account)
}

#[test]
fn erc20() {
    let spec_id = SpecId::LATEST;
    let block_env = BlockEnv::default();
    let block_size = 100_000; // number of transactions

    // Mock the beneficiary account (`Address:ZERO`) and the next `block_size` user accounts.
    let mut accounts: Vec<(Address, Account)> =
        (0..=block_size).map(common::mock_account).collect();
    let user_addresses: Vec<Address> = (1..=block_size).map(|idx| accounts[idx].0).collect();

    // Mock the ERC20 account and give each user account a token
    let (erc20_address, erc20_account) = mock_erc20_account(&user_addresses);
    accounts.push((erc20_address, erc20_account));

    common::test_txs(
        &accounts,
        spec_id,
        block_env,
        // Mock `block_size` transactions sending some tokens to itself.
        // Skipping `Address::ZERO` as the beneficiary account.
        user_addresses
            .iter()
            .map(|address| TxEnv {
                caller: *address,
                transact_to: TransactTo::Call(erc20_address),
                gas_limit: 65536u64,
                gas_price: U256::from(0xb2d05e07u64),
                data: Bytes::from(
                    [
                        // MethodID: transfer
                        fixed_bytes!("a9059cbb").as_slice(),
                        // address _to
                        B256::left_padding_from(address.as_slice()).as_slice(),
                        // uint256 value
                        B256::from(U256::from(1)).as_slice(),
                    ]
                    .concat(),
                ),
                ..TxEnv::default()
            })
            .collect(),
    );
}
