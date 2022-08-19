# grpc-health-check

grpc-health-check is a minimal, high performance, memory-friendly, safe implementation of the gRPC health checking protocol.

grpc-health-check provides a simple command line that eases the transition from other tools and requires no dependencies. 
Most grpc-health-check commands can be run as a regular user, without requiring additional privileges.

Exits with a status of 0 (true) or 1 (false), depending on the evaluation of the gRPC health checking protocol status.

[![CI](https://github.com/paypizza/grpc-health-check/workflows/CI/badge.svg?branch=master)](https://github.com/paypizza/grpc-health-check/actions)
[![codecov](https://codecov.io/gh/paypizza/grpc-health-check/branch/master/graph/badge.svg?token=SMK4DX1SDR)](https://codecov.io/gh/paypizza/grpc-health-check)

## Documentation

* [Official documentation](https://github.com/grpc/grpc/blob/master/doc/health-checking.md)

## Usage

```sh
$ grpc-health-check [FLAGS] [OPTIONS]
```

Example

```sh
$ grpc-health-check --port 5400
```

## Installation

### Kubernetes

```yaml
spec:
  containers:
  - name: server
    image: ...
    ports:
    - containerPort: 5400
    readinessProbe:
      exec:
        command: ["grpc-health-check", "--port=5400"]
      initialDelaySeconds: 2
    livenessProbe:
      exec:
        command: ["grpc-health-check", "--port=5400"]
      periodSeconds: 1 // Check once every second
      initialDelaySeconds: 4
```

### Linux

#### Alpine

```sh
$ apk add grpc-health-check --repository=http://dl-cdn.alpinelinux.org/alpine/edge/testing
```

#### Fedora

```sh
$ dnf copr enable paypizza/community
$ dnf install grpc-health-check
```

#### RHEL/CentOS

```sh
$ dnf copr enable paypizza/community
$ dnf install grpc-health-check
```

#### Ubuntu

```sh
$ add-apt-repository ppa:paypizza/community
$ apt-get install grpc-health-check
```

#### openSUSE

```sh
$ zypper ar https://copr.fedorainfracloud.org/coprs/paypizza/community/repo/opensuse-leap/
$ zypper in grpc-health-check
```

## Flags

**--help**, **-h**

Prints help information.

**--version**, **-V**

Prints version information.

**--verbose**

Enable verbose mode.

## Options

**--address**=*address*

Address to the gRPC server (default *localhost*).

**--config**=*config*

File path to the YAML configuration file.

**--port**=*port*

Port to the gRPC server.

**--rpc-timeout**=*rpc-timeout*

Timeout for establishing connection in milliseconds (ms) (default *1500*).

**--service-name**=*service-name*

Name of the service to check (default *""*). An empty string is the default value, according to the gRPC Health Check Protocol specification.

**--stream**=*true|false*

Enable RPC streaming mode (default *false*).

**--tls-ca-cert**=*tls-ca-cert*

File path to the CA certificate against which to verify the server's certificate.

**--tls-client-cert**=*tls-client-cert*

File path to the client's certificate.

**--tls-client-key**=*tls-client-key*

File path to the client's private key.

**--tls-domain-name**=*tls-domain-name*

Name of the domain against which to verify the server's certificate.

**--user-agent**=*user-agent*

Name of the user agent.

## Configuration

YAML

```yaml
address: ""
port: integer
rpc_timeout: integer
service_name: ""
stream: true|false
tls_ca_cert: ""
tls_client_cert: ""
tls_client_key: ""
tls_domain_name: ""
user_agent: ""
```
