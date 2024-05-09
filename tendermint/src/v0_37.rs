pub mod abci {
    include!("./prost/v0_37/tendermint.abci.rs");
    #[cfg(feature = "client")]
    pub use abci_application_client::AbciApplicationClient;
    #[cfg(feature = "server")]
    pub use abci_application_server::{AbciApplication, AbciApplicationServer};
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
