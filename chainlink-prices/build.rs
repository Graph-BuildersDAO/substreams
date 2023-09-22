use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("ChainlinkAggregator", "abi/chainlink_aggregator.json")?
        .generate()?
        .write_to_file("src/abi/chainlink_aggregator.rs")?;
    Abigen::new("PriceFeed", "abi/price_feed.json")?
        .generate()?
        .write_to_file("src/abi/price_feed.rs")?;
    Abigen::new("Erc20", "abi/erc20.json")?
        .generate()?
        .write_to_file("src/abi/erc20.rs")?;
    Abigen::new("PriceProvider", "abi/price_provider.json")?
        .generate()?
        .write_to_file("src/abi/price_provider.rs")?;
    Abigen::new("FeedRegistry", "abi/feed_registry.json")?
        .generate()?
        .write_to_file("src/abi/feed_registry.rs")?;
    Ok(())
}