//! Integration tests for ABCI client/server.

#[cfg(all(feature = "client", feature = "v0_37"))]
mod evmos_abci {
    use common_tendermint::v0_37::abci::{AbciApplicationClient, RequestEcho, ResponseEcho};
    use std::error::Error;
    #[tokio::test]
    async fn abci_echo() -> Result<(), Box<dyn Error>> {
        let avmos_server = "http://192.168.1.254:26618";
        let echo_message = "Hello abci";
        let mut client = AbciApplicationClient::connect(avmos_server).await?;
        let request = tonic::Request::new(RequestEcho {
            message: String::from(echo_message),
        });
        let response = client.echo(request).await;
        assert!(response.is_ok());
        let response = response.unwrap();
        println!("RESPONSE={:?}", &response);
        let ResponseEcho { message } = response.into_inner();
        assert_eq!(message, echo_message);

        Ok(())
    }
}
