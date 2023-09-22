use std::collections::HashMap;
use crate::pb::chainlink::v1::Erc20Token;

lazy_static::lazy_static! {
    pub static ref TOKEN_DENOMS: HashMap<String, Erc20Token> = {
        //This needs to be maintained as more assets are added. See here as a reference: https://docs.chain.link/data-feeds/feed-registry
        
        let mapping: HashMap<String, Erc20Token> = HashMap::from([
            ("eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee".to_string(), Erc20Token {
                address: "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee".to_string(),
                symbol: "ETH".to_string(),
                name: "Ether".to_string(),
                decimals: 18
            }),
            ("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb".to_string(), Erc20Token {
                address: "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb".to_string(),
                symbol: "BTC".to_string(),
                name: "Bitcoin".to_string(),
                decimals: 8
            }),
            ("0000000000000000000000000000000000000348".to_string(), Erc20Token {
                address: "0000000000000000000000000000000000000348".to_string(),
                symbol: "USD".to_string(),
                name: "US Dollar".to_string(),
                decimals: 8
            }),
            ("0000000000000000000000000000000000000036".to_string(), Erc20Token {
                address: "0000000000000000000000000000000000000036".to_string(),
                symbol: "AUD".to_string(),
                name: "Australian Dollar".to_string(),
                decimals: 8
            }),
            ("0000000000000000000000000000000000000124".to_string(), Erc20Token {
                address: "0000000000000000000000000000000000000124".to_string(),
                symbol: "CAD".to_string(),
                name: "Canadian Dollar".to_string(),
                decimals: 8
            }),
            ("0000000000000000000000000000000000000756".to_string(), Erc20Token {
                address: "0000000000000000000000000000000000000756".to_string(),
                symbol: "CHF".to_string(),
                name: "Swiss Franc".to_string(),
                decimals: 8
            }),
            ("0000000000000000000000000000000000000156".to_string(), Erc20Token {
                address: "0000000000000000000000000000000000000156".to_string(),
                symbol: "CNY".to_string(),
                name: "Chinese Yuan".to_string(),
                decimals: 8
            }),
            ("0000000000000000000000000000000000000978".to_string(), Erc20Token {
                address: "0000000000000000000000000000000000000978".to_string(),
                symbol: "EUR".to_string(),
                name: "Euro".to_string(),
                decimals: 8
            }),
            ("0000000000000000000000000000000000000826".to_string(), Erc20Token {
                address: "0000000000000000000000000000000000000826".to_string(),
                symbol: "GBP".to_string(),
                name: "Pound Sterling".to_string(),
                decimals: 8
            }),
            ("0000000000000000000000000000000000000356".to_string(), Erc20Token {
                address: "0000000000000000000000000000000000000356".to_string(),
                symbol: "INR".to_string(),
                name: "Indian Rupee".to_string(),
                decimals: 8
            }),
            ("0000000000000000000000000000000000000392".to_string(), Erc20Token {
                address: "0000000000000000000000000000000000000392".to_string(),
                symbol: "JPY".to_string(),
                name: "Japanese Yen".to_string(),
                decimals: 8
            }),
            ("0000000000000000000000000000000000000410".to_string(), Erc20Token {
                address: "0000000000000000000000000000000000000410".to_string(),
                symbol: "KRW".to_string(),
                name: "South Korean Won".to_string(),
                decimals: 8
            }),
            ("0000000000000000000000000000000000000554".to_string(), Erc20Token {
                address: "0000000000000000000000000000000000000554".to_string(),
                symbol: "NZD".to_string(),
                name: "New Zealand Dollar".to_string(),
                decimals: 8
            }),
            ("0000000000000000000000000000000000000702".to_string(), Erc20Token {
                address: "0000000000000000000000000000000000000702".to_string(),
                symbol: "SGD".to_string(),
                name: "Singapore Dollar".to_string(),
                decimals: 8
            }),
            ("0000000000000000000000000000000000000949".to_string(), Erc20Token {
                address: "0000000000000000000000000000000000000949".to_string(),
                symbol: "TRY".to_string(),
                name: "Turkish Lira".to_string(),
                decimals: 8
            }),
        ]);
        mapping
    };
}