use crate::acme::{Transaction, TransactionList, BlockHeader, ContractList, Contract};
use crate::tables::Tables;

pub fn register_transaction(tables: &mut Tables, transactions: &TransactionList) {
    for transaction in &transactions.transaction_details_list {
        create_transaction_entity(tables, transaction);
    }
}

fn create_transaction_entity(tables: &mut Tables, transaction: &Transaction) {
    tables
        .create_row("Transaction",  &transaction.id)
        .set("id", &transaction.id)
        .set("status", &transaction.status)
        .set("index",transaction.index)
        .set("gasUsed",  transaction.gas_used)
        .set("gasLimit",  transaction.gas_limit)
        .set("nonce",  transaction.nonce)
        .set("to",  transaction.to.clone())
        .set("from",  transaction.from.clone())
        .set("maxFeePerGas",  transaction.max_fee_per_gas.clone())
        .set("maxPriorityFeePerGas",  transaction.max_priority_fee_per_gas.clone())
        .set("blockNumber",  transaction.block_number)
        .set("value",  transaction.value.clone())
        .set("timestamp",  transaction.timestamp)
        .set("gasPrice",  transaction.gas_price);
}

pub fn register_contracts(tables: &mut Tables, contracts: &ContractList) {
    for contract in &contracts.contract_list {
        create_contract_entity(tables, contract);
    }
}

fn create_contract_entity(tables: &mut Tables, contract: &Contract) {
    tables
        .create_row("Contract",  &contract.transaction_hash.clone())
        .set("id", &contract.address)
        .set("address", &contract.address)
        .set("owner", &contract.owner)
        .set("transactionHash",contract.transaction_hash.clone())
        .set("blockNumber",  contract.block_number)
        .set("timestamp",  contract.timestamp);
}

pub fn create_block_entity(tables: &mut Tables, block:&BlockHeader) {
    tables
        .create_row("Block", &block.id)
        .set("id", format!("0x{}", &block.id))
        .set("parentHash", &block.parent_hash)
        .set("uncleHash", &block.uncle_hash)
        .set("nonce", block.nonce)
        .set("receiptRoot", &block.receipt_root)
        .set("number", block.number)
        .set("gasLimit", block.gas_limit)
        .set("gasUsed", block.gas_used)
        .set("timestamp", block.timestamp)
        .set("size", block.size)
        .set("difficulty", &block.difficulty)
        .set("totalDifficulty", &block.total_difficulty);
}

