//! Integration tests for ABCI client/server.

#[cfg(all(feature = "client", feature = "v0_37"))]
mod evmos_abci {
    use alloy_primitives::hex;
    use common_tendermint::v0_37::abci::{
        AbciApplicationClient, RequestCheckTx, RequestEcho, ResponseCheckTx, ResponseEcho,
    };
    use std::error::Error;
    const EVMOS_SERVER: &str = "http://localhost:26618";
    #[tokio::test]
    async fn abci_echo() -> Result<(), Box<dyn Error>> {
        let echo_message = "Hello abci";
        let mut client = AbciApplicationClient::connect(EVMOS_SERVER).await?;
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

    #[tokio::test]
    async fn abci_check_tx() -> Result<(), Box<dyn Error>> {
        let mut client = AbciApplicationClient::connect(EVMOS_SERVER).await?;
        let raw_tx = hex!("02f876820a28808477359400847735940082520894ab0840c0e43688012c1adb0f5e3fc665188f83d28a029d394a5d630544000080c080a0a044076b7e67b5deecc63f61a8d7913fab86ca365b344b5759d1fe3563b4c39ea019eab979dd000da04dfc72bb0377c092d30fd9e1cab5ae487de49586cc8b0090");

        let request = tonic::Request::new(RequestCheckTx {
            tx: raw_tx.to_vec(),
            r#type: 1,
        });
        let response: Result<tonic::Response<ResponseCheckTx>, tonic::Status> =
            client.check_tx(request).await;
        println!("{:?}", &response);
        assert!(response.is_ok());
        let response = response.unwrap();
        println!("RESPONSE={:?}", &response);
        let ResponseCheckTx {
            code,
            data,
            log,
            info,
            gas_wanted,
            gas_used,
            events,
            codespace,
            sender,
            priority,
            mempool_error,
        } = response.into_inner();

        Ok(())
    }
}
