use crate::{
    pb::schema::{Approvals, Transfers, Uris},
    ADDRESS,
};
use substreams::Hex;
use substreams_entity_change::tables::Tables;

pub fn transfers_to_table_changes(tables: &mut Tables, transfers: &Transfers) {
    for transfer in transfers.transfers.iter() {
        // handle the transfer
        let key = &transfer.id;
        let row = tables.update_row("Transfer", key);
        row.set("from", &transfer.from);
        row.set("to", &transfer.to);
        row.set("operator", &transfer.operator);
        row.set("tokenIds", &transfer.token_ids);
        if &transfer.token_ids.len() == &1 {
            row.set("transferType", "SINGLE");
        } else {
            row.set("transferType", "BATCH");
        }

        // handle the accounts
        tables.create_row("Account", &transfer.from);
        tables.create_row("Account", &transfer.to);

        // handle updating the token owner
        for token in transfer.token_ids.iter() {
            tables
                .update_row("Token", token)
                .set("collection", ADDRESS.to_string())
                .set("owner", &transfer.to);
        }
    }
}

pub fn approvals_to_table_changes(tables: &mut Tables, approvals: &Approvals) {
    for approval in approvals.approvals.iter() {
        // handle the approval
        let key = &approval.id;
        let row = tables.update_row("ApprovalForAll", key);
        row.set("owner", &approval.owner);
        row.set("operator", &approval.operator);
        row.set("approved", &approval.approved);

        // handle creation of accounts
        tables.create_row("Account", &approval.owner);
        tables.create_row("Account", &approval.operator);
    }
}

pub fn uris_to_table_changes(tables: &mut Tables, uris: &Uris) {
    for uri in uris.uris.iter() {
        // handle the uri
        let row = tables.update_row("Token", &uri.token_id);
        row.set("uri", &uri.value);
    }
}

pub fn format_hex(address: &[u8]) -> String {
    format!("0x{}", Hex(address).to_string())
}
