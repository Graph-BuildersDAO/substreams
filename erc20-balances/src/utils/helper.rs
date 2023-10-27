
use crate::abi;

use abi::erc20::functions;
use substreams::scalar::BigInt;
use substreams::Hex;
use crate::pb::erc20::types::v1::Erc20Token;


pub fn append_0x(i: &str) -> String {
    format!("0x{}", i)
}

pub fn get_erc20_token(token_address: String) -> Option<Erc20Token> {
    let token_address_vec = Hex::decode(token_address.clone()).unwrap();

    let name = functions::Name {}
        .call(token_address_vec.clone())
        .unwrap_or(String::new());
    let symbol = functions::Symbol {}
        .call(token_address_vec.clone())
        .unwrap_or(String::new());
    let decimals = functions::Decimals {}
        .call(token_address_vec.clone())
        .unwrap_or(BigInt::zero())
        .to_u64();

    Some(Erc20Token {
        address: token_address.clone(),
        name: name,
        symbol: symbol,
        decimals: decimals,
    })
}