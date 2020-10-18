% grpc-health-check(1)

## NAME
grpc\-health\-check - gRPC health checking protocol

## SYNOPSIS
**grpc-health-check** [*flags*] [*options*]

## DESCRIPTION
grpc-health-check is a minimal, high performance, memory-friendly, safe implementation of 
the gRPC health checking protocol.

grpc-health-check provides a simple command line that eases the transition from other
tools and requires no dependencies.
Most grpc-health-check commands can be run as a regular user, without requiring additional
privileges.

Exits with a status of 0 (true) or 1 (false), depending on the evaluation of the gRPC health checking protocol status.
## FLAGS

**--help**, **-h**

Prints help information.

**--version**, **-V**

Prints version information.

**--verbose**

Enable verbose mode.
## OPTIONS

**--address**=*address*

Address to the gRPC server (default *[::1]*).

**--config**=*config*

File path to the YAML configuration file.

**--port**=*port*

Port to the gRPC server.

**--rpc-timeout**=*rpc-timeout*

RPC timeout in milliseconds (ms) (default *1500*).

**--service-name**=*service-name*

Name of the service to check (default *""*). An empty string is the default value, according to the gRPC health specification.

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