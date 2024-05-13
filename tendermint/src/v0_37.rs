pub mod abci {
    include!("./prost/v0_37/tendermint.abci.rs");
    #[cfg(feature = "client")]
    pub use abci_application_client::AbciApplicationClient;
    #[cfg(feature = "server")]
    pub use abci_application_server::{AbciApplication, AbciApplicationServer};
}
pub mod rpc {
    pub mod grpc {
        include!("./prost/v0_37/tendermint.rpc.grpc.rs");
        #[cfg(feature = "client")]
        pub use broadcast_api_client::BroadcastApiClient;
        #[cfg(feature = "server")]
        pub use broadcast_api_server::{BroadcastApi, BroadcastApiServer};
    }
}
pub mod crypto {
    include!("./prost/v0_37/tendermint.crypto.rs");
}
pub mod types {
    include!("./prost/v0_37/tendermint.types.rs");
}
pub mod version {
    include!("./prost/v0_37/tendermint.version.rs");
}
