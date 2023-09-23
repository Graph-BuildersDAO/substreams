// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigInt {
    #[prost(bytes="vec", tag="1")]
    pub bytes: ::prost::alloc::vec::Vec<u8>,
}
/// ## Block Details ##
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockHeader {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub parent_hash: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub uncle_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub nonce: u64,
    #[prost(bytes="vec", tag="5")]
    pub receipt_root: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="6")]
    pub number: u64,
    #[prost(uint64, tag="7")]
    pub gas_limit: u64,
    #[prost(uint64, tag="8")]
    pub gas_used: u64,
    #[prost(int64, tag="9")]
    pub timestamp: i64,
    #[prost(uint64, tag="10")]
    pub size: u64,
    #[prost(string, tag="11")]
    pub difficulty: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub total_difficulty: ::prost::alloc::string::String,
}
/// ## Transaction List ##
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionList {
    #[prost(message, repeated, tag="1")]
    pub transaction_details_list: ::prost::alloc::vec::Vec<Transaction>,
}
/// ## Transaction Details ##
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub status: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub gas_used: u64,
    #[prost(uint64, tag="4")]
    pub gas_limit: u64,
    #[prost(uint64, tag="5")]
    pub nonce: u64,
    #[prost(string, tag="6")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub from: ::prost::alloc::string::String,
    #[prost(uint64, tag="8")]
    pub max_fee_per_gas: u64,
    #[prost(uint64, tag="9")]
    pub max_priority_fee_per_gas: u64,
    #[prost(uint32, tag="10")]
    pub index: u32,
    #[prost(uint64, tag="11")]
    pub value: u64,
    #[prost(uint64, tag="12")]
    pub block_number: u64,
    #[prost(int64, tag="13")]
    pub timestamp: i64,
    #[prost(uint64, tag="14")]
    pub gas_price: u64,
}
/// ## Contracts List ##
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractList {
    #[prost(message, repeated, tag="1")]
    pub contract_list: ::prost::alloc::vec::Vec<Contract>,
}
/// ## Contract ##
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contract {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub transaction_hash: ::prost::alloc::string::String,
    #[prost(int64, tag="4")]
    pub timestamp: i64,
    #[prost(uint64, tag="6")]
    pub block_number: u64,
}
// @@protoc_insertion_point(module)
