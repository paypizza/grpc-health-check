use assert_cmd::prelude::*;
use futures_util::FutureExt;
use integration_tests::pb::test_server;
use predicates::str::{contains, is_empty};
use std::net::SocketAddr;
use std::process::Command;
use std::time::Duration;
use tokio::sync::oneshot;
use tokio_stream::wrappers::TcpListenerStream;
use tokio_stream::StreamExt;
use tonic::transport::Server;

struct TestService;

#[tonic::async_trait]
impl test_server::Test for TestService {}

#[tokio::test]
async fn connect_success() {
    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<test_server::TestServer<TestService>>()
        .await;

    let (tx, rx) = oneshot::channel();

    let addr: SocketAddr = "[::1]:50051".parse().expect("SocketAddr parse");
    let listener = tokio::net::TcpListener::bind(addr).await.expect("bind");

    let jh = tokio::spawn(async move {
        Server::builder()
            .add_service(health_service)
            .add_service(test_server::TestServer::new(TestService))
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
            .failure()
            .stderr(contains("NotFound"));
    })
    .await
    .unwrap();

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
        .set_not_serving::<test_server::TestServer<TestService>>()
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
