// @generated
pub mod google {
    // @@protoc_insertion_point(attribute:google.protobuf)
    pub mod protobuf {
        include!("google.protobuf.rs");
        // @@protoc_insertion_point(google.protobuf)
    }
}
// @@protoc_insertion_point(attribute:schema)
pub mod schema {
    include!("schema.rs");
    // @@protoc_insertion_point(schema)
}
pub mod sf {
    pub mod ethereum {
        pub mod block_meta {
            // @@protoc_insertion_point(attribute:sf.ethereum.block_meta.v1)
            pub mod v1 {
                include!("sf.ethereum.block_meta.v1.rs");
                // @@protoc_insertion_point(sf.ethereum.block_meta.v1)
            }
        }
    }
    pub mod substreams {
        pub mod rpc {
            // @@protoc_insertion_point(attribute:sf.substreams.rpc.v2)
            pub mod v2 {
                include!("sf.substreams.rpc.v2.rs");
                // @@protoc_insertion_point(sf.substreams.rpc.v2)
            }
        }
        // @@protoc_insertion_point(attribute:sf.substreams.v1)
        pub mod v1 {
            include!("sf.substreams.v1.rs");
            // @@protoc_insertion_point(sf.substreams.v1)
        }
    }
}
pub mod substreams {
    pub mod entity {
        // @@protoc_insertion_point(attribute:substreams.entity.v1)
        pub mod v1 {
            include!("substreams.entity.v1.rs");
            // @@protoc_insertion_point(substreams.entity.v1)
        }
    }
}
