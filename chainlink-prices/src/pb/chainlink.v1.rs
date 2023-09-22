// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetPair {
    #[prost(string, tag="1")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub aggregator_address: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub oracle_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub base_token: ::core::option::Option<Erc20Token>,
    #[prost(message, optional, tag="5")]
    pub quote_token: ::core::option::Option<Erc20Token>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Price {
    #[prost(message, optional, tag="1")]
    pub asset_pair: ::core::option::Option<AssetPair>,
    #[prost(string, tag="2")]
    pub price: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub raw_price: ::prost::alloc::string::String,
    #[prost(int64, tag="4")]
    pub timestamp: i64,
    #[prost(string, tag="5")]
    pub transmitter: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetPairs {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<AssetPair>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Prices {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<Price>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Erc20Token {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub decimals: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Aggregator {
    #[prost(message, optional, tag="1")]
    pub base_asset: ::core::option::Option<Erc20Token>,
    #[prost(message, optional, tag="2")]
    pub quote_asset: ::core::option::Option<Erc20Token>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Aggregators {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<Aggregator>,
}
// @@protoc_insertion_point(module)
