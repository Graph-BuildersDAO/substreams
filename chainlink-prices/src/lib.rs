mod abi;
mod helper;
mod pb;
mod utils;

use std::str::FromStr;
use substreams::errors::Error;
use substreams::scalar::{BigDecimal, BigInt};
use substreams::store::StoreNew;
use substreams::store::{StoreGet, StoreGetProto, StoreSetIfNotExistsProto};
use substreams::store::{StoreSet, StoreSetBigInt, StoreSetIfNotExists};
use substreams::Hex;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;
use substreams_ethereum::pb::eth::v2::{self as eth};
use substreams_ethereum::Event;

use crate::abi::chainlink_aggregator;
use crate::abi::feed_registry;
use crate::pb::chainlink::v1::{Aggregator, AssetPair, Erc20Token, Price, Prices};

#[substreams::handlers::store]
fn store_confirmed_feeds(block: eth::Block, output: StoreSetIfNotExistsProto<Aggregator>) {
    for log in block.logs() {
        if let Some(event) = feed_registry::events::FeedConfirmed::match_and_decode(log) {
            let latest_aggregator = Hex(event.clone().latest_aggregator).to_string();
            let asset = Hex(event.clone().asset).to_string();
            let denomination = Hex(event.clone().denomination).to_string();

            let asset_call;
            let denomination_call;

            if let Some(token) = utils::TOKEN_DENOMS.get(&asset.to_string()).cloned() {
                asset_call = token;
            } else {
                asset_call = helper::get_erc20_token(asset).unwrap();
            }

            if let Some(token) = utils::TOKEN_DENOMS.get(&denomination.to_string()).cloned() {
                denomination_call = token;
            } else {
                denomination_call = helper::get_erc20_token(denomination).unwrap();
            }

            let aggregator_pair = Aggregator {
                base_asset: Some(Erc20Token {
                    address: asset_call.address,
                    name: asset_call.name,
                    symbol: asset_call.symbol,
                    decimals: asset_call.decimals,
                }),
                quote_asset: Some(Erc20Token {
                    address: denomination_call.address,
                    name: denomination_call.name,
                    symbol: denomination_call.symbol,
                    decimals: denomination_call.decimals,
                }),
            };

            output.set_if_not_exists(0, &latest_aggregator, &aggregator_pair)
        }
    }
}

#[substreams::handlers::map]
fn get_chainlink_answers(
    block: eth::Block,
    confirmed_feeds: StoreGetProto<Aggregator>,
) -> Result<Prices, substreams::errors::Error> {
    
    let mut prices: Prices = Prices { items: vec![] };

    for logs in block.logs() {

        substreams::log::println(&format!(
            "To: {:?}",
            Hex(logs.log.clone().address).to_string()
        ));

        substreams::log::println(&format!(
            "TX: {}",
            Hex(logs.receipt.transaction.clone().hash).to_string()
        ));

        substreams::log::println(&format!(
            "To: {}",
            Hex(logs.receipt.transaction.clone().to).to_string()
        ));

        substreams::log::println(&format!(
            "From: {}",
            Hex(logs.receipt.transaction.clone().from).to_string()
        ));

        if let Some(event) = chainlink_aggregator::events::AnswerUpdated::match_and_decode(logs.log) {
            let aggregator_lookup = confirmed_feeds.get_last(Hex(logs.log.clone().address).to_string());
            let found_aggregator = aggregator_lookup.is_some();

            // let emitted_to = helper::transform_address(trx.clone().to);
            // let emitted_from = helper::transform_address(trx.clone().from);

            // substreams::log::println(&format!("{} {}", emitted_to, emitted_from));

            if found_aggregator {
                substreams::log::println(
                    aggregator_lookup
                        .clone()
                        .unwrap()
                        .base_asset
                        .unwrap()
                        .address,
                );
            } else {
                substreams::log::println("No aggregator found")
            }

            let description = chainlink_aggregator::functions::Description {}
                .call(logs.log.clone().address.to_vec())
                .unwrap_or(String::new());

            let decimals = chainlink_aggregator::functions::Decimals {}
                .call(logs.log.clone().address.to_vec())
                .unwrap_or(BigInt::zero());

            if !description.is_empty() {
                let token_price = event.current.to_decimal(decimals.to_u64());
                // substreams::log::println(Hex(log.clone().address).to_string());
                let base_token = aggregator_lookup.clone().unwrap_or_default().base_asset;
                let quote_token = aggregator_lookup.clone().unwrap_or_default().quote_asset;

                prices.items.push(Price {
                    asset_pair: Some(AssetPair {
                        description: description.clone().replace(" ", "").to_string(),
                        aggregator_address: Hex(logs.log.clone().address).to_string(),
                        oracle_address: Hex(logs.receipt.transaction.from.clone()).to_string(),
                        base_token: base_token,
                        quote_token: quote_token,
                    }),
                    price: token_price.to_string(),
                    raw_price: event.current.to_string(),
                    timestamp: block.timestamp_seconds() as i64,
                    transmitter: Hex(logs.receipt.transaction.from.clone()).to_string(),
                });
            }
        }
    }
    substreams::log::println("Block Finished");
    Ok(prices)
}

#[substreams::handlers::store]
fn chainlink_price_store(events: Prices, output: StoreSetBigInt) {
    for price in events.items {
        let asset_description = price.clone().asset_pair.unwrap();

        let split_description: Vec<String> = asset_description
            .description
            .split("/")
            .map(String::from)
            .collect();

        if price.clone().asset_pair.is_some() {
            if price.clone().asset_pair.unwrap().base_token.is_some()
                && price.clone().asset_pair.unwrap().quote_token.is_some()
            {
                output.set(
                    0,
                    format!(
                        "price_by_address:{}:{}",
                        price
                            .clone()
                            .asset_pair
                            .unwrap()
                            .base_token
                            .unwrap()
                            .address,
                        price
                            .clone()
                            .asset_pair
                            .unwrap()
                            .quote_token
                            .unwrap()
                            .address
                    ),
                    &BigInt::from_str(&price.raw_price).unwrap(),
                );
            }
        }

        output.set(
            0,
            format!(
                "price_by_aggregator:{}",
                price.clone().asset_pair.unwrap().aggregator_address
            ),
            &BigInt::from_str(&price.raw_price).unwrap(),
        );

        output.set(
            0,
            format!("price_by_symbol:{}", split_description.join(":")),
            &BigInt::from_str(&price.raw_price).unwrap(),
        );
    }
}

#[substreams::handlers::map]
fn graph_out(events: Prices) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();

    for event in events.items {
        let key = format!(
            "0x{}",
            Hex(event.timestamp.to_be_bytes())
                .to_string()
                .trim_start_matches('0')
        );

        tables
            .create_row(
                "Price",
                &format!(
                    "{}:{}",
                    &event.asset_pair.as_ref().unwrap().description,
                    key
                ),
            )
            .set("price", BigDecimal::try_from(&event.price).unwrap())
            .set("rawPrice", BigInt::try_from(&event.raw_price).unwrap())
            .set(
                "timestamp",
                BigInt::try_from(&event.timestamp.to_string()).unwrap(),
            )
            .set("assetPair", &event.asset_pair.as_ref().unwrap().description)
            .set(
                "transmitter",
                format!("0x{}", &event.asset_pair.as_ref().unwrap().oracle_address),
            )
            .set(
                "aggregator",
                format!(
                    "0x{}",
                    &event.asset_pair.as_ref().unwrap().aggregator_address
                ),
            );

        tables
            .create_row(
                "AssetPair",
                &format!("{}", &event.asset_pair.as_ref().unwrap().description),
            )
            .set("currentPrice", BigDecimal::try_from(&event.price).unwrap())
            .set(
                "currentRawPrice",
                BigInt::try_from(&event.raw_price).unwrap(),
            )
            .set(
                "aggregator",
                format!(
                    "0x{}",
                    &event.asset_pair.as_ref().unwrap().aggregator_address
                ),
            );
        tables.create_row(
            "Aggregator",
            format!(
                "0x{}",
                &event.asset_pair.as_ref().unwrap().aggregator_address
            ),
        );

        if event.asset_pair.as_ref().unwrap().base_token.is_some() {
            tables
                .create_row(
                    "Asset",
                    &format!(
                        "0x{}",
                        &event
                            .asset_pair
                            .as_ref()
                            .unwrap()
                            .base_token
                            .as_ref()
                            .unwrap()
                            .address
                    ),
                )
                .set(
                    "symbol",
                    &format!(
                        "{}",
                        &event
                            .asset_pair
                            .as_ref()
                            .unwrap()
                            .base_token
                            .as_ref()
                            .unwrap()
                            .symbol
                    ),
                )
                .set(
                    "name",
                    &format!(
                        "{}",
                        &event
                            .asset_pair
                            .as_ref()
                            .unwrap()
                            .base_token
                            .as_ref()
                            .unwrap()
                            .name
                    ),
                )
                .set(
                    "decimals",
                    BigInt::try_from(
                        &event
                            .asset_pair
                            .as_ref()
                            .unwrap()
                            .base_token
                            .as_ref()
                            .unwrap()
                            .decimals
                            .to_string(),
                    )
                    .unwrap(),
                );
            tables
                .update_row(
                    "AssetPair",
                    &format!("{}", &event.asset_pair.as_ref().unwrap().description),
                )
                .set(
                    "asset",
                    &format!(
                        "0x{}",
                        &event
                            .asset_pair
                            .as_ref()
                            .unwrap()
                            .base_token
                            .as_ref()
                            .unwrap()
                            .address
                    ),
                )
        } else {
            continue;
        };

        if event.asset_pair.as_ref().unwrap().quote_token.is_some() {
            tables
                .create_row(
                    "Asset",
                    &format!(
                        "0x{}",
                        &event
                            .asset_pair
                            .as_ref()
                            .unwrap()
                            .quote_token
                            .as_ref()
                            .unwrap()
                            .address
                    ),
                )
                .set(
                    "symbol",
                    &format!(
                        "{}",
                        &event
                            .asset_pair
                            .as_ref()
                            .unwrap()
                            .quote_token
                            .as_ref()
                            .unwrap()
                            .symbol
                    ),
                )
                .set(
                    "name",
                    &format!(
                        "{}",
                        &event
                            .asset_pair
                            .as_ref()
                            .unwrap()
                            .quote_token
                            .as_ref()
                            .unwrap()
                            .name
                    ),
                )
                .set(
                    "decimals",
                    BigInt::try_from(
                        &event
                            .asset_pair
                            .as_ref()
                            .unwrap()
                            .quote_token
                            .as_ref()
                            .unwrap()
                            .decimals
                            .to_string(),
                    )
                    .unwrap(),
                );
            tables
                .update_row(
                    "AssetPair",
                    &format!("{}", &event.asset_pair.as_ref().unwrap().description),
                )
                .set(
                    "comparedAsset",
                    &format!(
                        "0x{}",
                        &event
                            .asset_pair
                            .as_ref()
                            .unwrap()
                            .quote_token
                            .as_ref()
                            .unwrap()
                            .address
                    ),
                )
        } else {
            continue;
        };
    }
    Ok(tables.to_entity_changes())
}
