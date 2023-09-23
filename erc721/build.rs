use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("erc721", "abis/erc721.json")
        .unwrap()
        .generate()
        .unwrap()
        .write_to_file("src/abi/erc721.rs")
}
