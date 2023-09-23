#[path = "./abi/erc721.rs"]
mod erc721;

mod helpers;
mod pb;

use pb::schema::{Approval, Approvals, Transfer, Transfers};
use substreams::pb::substreams::Clock;
use substreams_entity_change::{pb::entity::EntityChanges, tables::Tables};
use substreams_ethereum::{pb::eth, Event};

use helpers::*;

use erc721::events::{Approval as ApprovalEvent, Transfer as TransferEvent};

pub const ADDRESS: &str = "0xBC4CA0EdA7647A8aB7C2061c2E118A18a936f13D";
const START_BLOCK: u64 = 12287507;

#[substreams::handlers::map]
fn map_transfers(block: eth::v2::Block) -> Result<Transfers, substreams::errors::Error> {
    let transfers = block
        .logs()
        .filter_map(|log| {
            if format_hex(log.address()) == ADDRESS.to_lowercase() {
                if let Some(transfer) = TransferEvent::match_and_decode(log) {
                    Some((transfer, format_hex(&log.receipt.transaction.hash)))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .map(|(transfer, hash)| Transfer {
            from: format_hex(&transfer.from),
            to: format_hex(&transfer.to),
            token_id: transfer.token_id.to_string(),
            tx_hash: hash,
        })
        .collect::<Vec<Transfer>>();

    Ok(Transfers { transfers })
}

#[substreams::handlers::map]
fn map_approvals(block: eth::v2::Block) -> Result<Approvals, substreams::errors::Error> {
    let approvals = block
        .logs()
        .filter_map(|log| {
            if format_hex(log.address()) == ADDRESS.to_lowercase() {
                if let Some(approval) = ApprovalEvent::match_and_decode(log) {
                    Some((approval, format_hex(&log.receipt.transaction.hash)))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .map(|(approval, hash)| Approval {
            owner: format_hex(&approval.owner),
            approved: format_hex(&approval.approved),
            token_id: approval.token_id.to_string(),
            tx_hash: hash,
        })
        .collect::<Vec<Approval>>();

    Ok(Approvals { approvals })
}

#[substreams::handlers::map]
pub fn graph_out(
    clock: Clock,
    transfers: Transfers,
    approvals: Approvals,
) -> Result<EntityChanges, substreams::errors::Error> {
    let mut tables = Tables::new();

    if clock.number == START_BLOCK {
        // Create the collection, we only need to do this once
        tables.create_row("Collection", ADDRESS.to_string());
    }

    transfers_to_table_changes(&mut tables, &transfers);

    approvals_to_table_changes(&mut tables, &approvals);

    Ok(tables.to_entity_changes())
}
