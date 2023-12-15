use crate::pb::erc20::types::v1::{BalanceChanges, Erc20Token};
use crate::utils::helper::{append_0x, get_erc20_token};
use substreams::scalar::BigInt;
use substreams::store::StoreGet;
use substreams::store::{StoreGetProto, StoreGetString};
use substreams::{errors::Error, pb::substreams::Clock};
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;

#[substreams::handlers::map]
pub fn graph_out(
    clock: Clock,
    block: BalanceChanges,
    token: StoreGetString,
) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();
    let block_num = clock.number.to_string();
    let timestamp = clock.timestamp.unwrap().seconds.to_string();

    for storage_change in block.balance_changes {
        let token_lookup = token.get_last(&storage_change.contract);
        let token_found = token_lookup.is_some();

        if token_found {
            let token = &get_erc20_token(storage_change.contract.clone()).unwrap();
            tables
                .create_row("Token", append_0x(&storage_change.contract))
                .set("name", token.name.clone())
                .set("decimals", token.decimals.clone())
                .set("symbol", token.symbol.clone());
        }

        let id = format!("{}:{}", storage_change.contract, storage_change.owner);

        if storage_change.change_type == 0 {
            continue;
        }

        tables.create_row("Account", append_0x(&storage_change.owner.clone()));

        tables
            .create_row("Balance", id)
            // contract address
            .set("token", append_0x(&storage_change.contract))
            // storage change
            .set("owner", append_0x(&storage_change.owner))
            .set(
                "balance",
                BigInt::try_from(storage_change.new_balance).unwrap_or_default(),
            );
    }

    Ok(tables.to_entity_changes())
}
