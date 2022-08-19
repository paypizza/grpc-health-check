#[cfg(test)]
mod test {
    use assert_cmd::prelude::*;
    use predicates::str::contains;

    use std::process::Command;

    #[test]
    fn no_args() -> Result<(), Box<dyn std::error::Error>> {
        Command::cargo_bin("grpc-health-check")?
            .arg("--verbose")
            .assert()
            .failure()
            .stderr(contains("undefined port"));

        Ok(())
    }

    #[test]
    fn invalid_port() -> Result<(), Box<dyn std::error::Error>> {
        Command::cargo_bin("grpc-health-check")?
            .args(&["--verbose", "--port", "a"])
            .assert()
            .failure();

        Command::cargo_bin("grpc-health-check")?
            .args(&["--verbose", "--port", "65536"])
            .assert()
            .failure();

        Ok(())
    }

    #[test]
    fn connect_returns_err() -> Result<(), Box<dyn std::error::Error>> {
        Command::cargo_bin("grpc-health-check")?
            .args(&["--port", "50052"])
            .assert()
            .failure()
            .stderr(contains("tonic::transport::Error"));

        Ok(())
    }
}
