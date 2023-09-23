# Generic ERC721 Substream Powered Subgraph

A simple subgraph for tracking a single erc721 smart contract.

Link to see what this is like live: [Here](https://thegraph.com/hosted-service/subgraph/mercuricchloride/erc721-substream)

# Quickstart

Make sure you have the latest versions of the following installed:
- Rust [Link to install](https://rustup.rs/)
- Graph-cli [Link to install](https://thegraph.com/docs/en/cookbook/quick-start/#2-install-the-graph-cli)
- substreams-cli [Link to install](https://substreams.streamingfast.io/getting-started/installing-the-cli)

## 1. Update the `ADDRESS` and `START_BLOCK` variables in `src/lib.rs`

``` rust
pub const ADDRESS: &str = "0xBC4CA0EdA7647A8aB7C2061c2E118A18a936f13D";
const START_BLOCK: u64 = 13090020;
```

## 2. Update the `initialBlock` params for all modules within `substreams.yaml`

``` yaml
  - name: map_transfers
    kind: map
    initialBlock: 13090020
    inputs:
      - source: sf.ethereum.type.v2.Block
    output:
      type: proto:schema.Transfers
```

## 3. Compile the Project with `cargo build --release --target wasm32-unknown-unknown`
We now need to recompile our WASM binary with the new changes we made to the rust files.

## 4. Pack the spkg with `substreams pack -o substream.spkg`
We need to bundle the protobuf definitions and the WASM binary into a single file. This is what we will deploy the subgraph.

## 5. Deploy the subgraph with `graph deploy <YOUR_ENDPOINT>`
This endpoint will change if you are deploying to the hosted service or decentralized network. But replace this with the command that is appropriate for your setup. 

## 6. Enjoy your data!

# Schema
    
``` graphql
type Collection @entity {
  id: ID! #address of the contract
  tokens: [Token!]! @derivedFrom(field: "collection") #tokens that belong to the contractA
}

type Transfer @entity {
  id: ID! #tx hash of the transfer
  from: Account! #account that sent the transfer
  to: Account! #account that received the transfer
  tokenId: Token! #token that was transferred
}

type Approval @entity {
  id: ID! #tx hash of the approval
  owner: Account!
  approved: Account!
  tokenId: Token!
}

type Token @entity {
  id: ID! #token id
  collection: Collection! #contract that the token belongs to
  owner: Account! #account that owns the token
  transfers: [Transfer!]! @derivedFrom(field: "tokenId") #transfers that the token has been involved in
  approvals: [Approval!]! @derivedFrom(field: "tokenId")
}

type Account @entity {
  id: ID! #Address of the account
  tokens: [Token!]! @derivedFrom(field: "owner") #tokens that the account owns
}
```




