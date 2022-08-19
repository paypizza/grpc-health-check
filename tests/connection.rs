#[cfg(test)]
mod test {
    use assert_cmd::prelude::*;
    use futures_util::FutureExt;
    use predicates::str::{contains, is_empty};
    use std::net::SocketAddr;
    use std::process::Command;
    use std::time::Duration;
    use tokio::sync::oneshot;
    use tokio_stream::wrappers::TcpListenerStream;
    use tonic::transport::Server;

    pub mod pb {
        #![allow(clippy::match_single_binding)]
        #![allow(clippy::derive_partial_eq_without_eq)]

        tonic::include_proto!("test");
    }

    struct TestService;

    #[tonic::async_trait]
    impl pb::test_server::Test for TestService {}

    #[tokio::test]
    async fn connect_success() {
        let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
        health_reporter
            .set_serving::<pb::test_server::TestServer<TestService>>()
            .await;

        let (tx, rx) = oneshot::channel();

        let addr: SocketAddr = "[::1]:50051".parse().expect("SocketAddr parse");
        let listener = tokio::net::TcpListener::bind(addr).await.expect("bind");

        let jh = tokio::spawn(async move {
            Server::builder()
                .add_service(health_service)
                .add_service(pb::test_server::TestServer::new(TestService))
                .serve_with_incoming_shutdown(TcpListenerStream::new(listener), rx.map(drop))
                .await
                .unwrap();
        });

        tokio::time::sleep(Duration::from_millis(100)).await;

        tokio::task::spawn_blocking(move || {
            Command::cargo_bin("grpc-health-check")
                .unwrap()
                .args(&["--verbose", "--port", "50051"])
                .assert()
                .success()
                .stdout(contains("Serving"));
        })
        .await
        .unwrap();

        tokio::task::spawn_blocking(move || {
            Command::cargo_bin("grpc-health-check")
                .unwrap()
                .args(&["--port", "50051", "--service-name", "test.Test"])
                .assert()
                .success()
                .stdout(is_empty());
        })
        .await
        .unwrap();

        tokio::time::sleep(Duration::from_millis(100)).await;

        health_reporter
            .set_not_serving::<pb::test_server::TestServer<TestService>>()
            .await;

        tokio::task::spawn_blocking(move || {
            Command::cargo_bin("grpc-health-check")
                .unwrap()
                .args(&[
                    "--verbose",
                    "--port",
                    "50051",
                    "--service-name",
                    "test.Test",
                ])
                .assert()
                .failure()
                .stdout(contains("NotServing"));
        })
        .await
        .unwrap();

        tokio::task::spawn_blocking(move || {
            Command::cargo_bin("grpc-health-check")
                .unwrap()
                .args(&["--port", "50051", "--service-name", "test.Test"])
                .assert()
                .failure()
                .stdout(is_empty());
        })
        .await
        .unwrap();

        tx.send(()).expect("send shutdown");
        jh.await.expect("server shutdown");
    }
}
