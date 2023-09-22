use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("erc1155", "abis/erc1155.json")
        .unwrap()
        .generate()
        .unwrap()
        .write_to_file("src/abi/erc1155.rs")
}
