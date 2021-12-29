# Keyrock Challenge

## Libraries used
#### App configuration
* ron
* serde
#### Async await
* tokio
#### gRPC + protobuf
#### Parsing data
* serde_json (will probably change)
#### Websocket client
* tokio-tungstenite
* tungstenite
* futures
#### Misc
* lazy_static
* url
* async-trait
* fast-float

## Modules
### configuration
Related to the initial static configuration of the app
### exceptions
Contains all the custom Errors (all based on the std::error::Error trait)
### exchanges
Exchanges streams and data representations
### grpc_server
Home of the gRPC server (wip)
### orderbook
The merged orderbook's data and functions to create it
### tests
All the tests for the above modules are here
### web
A web client to visualize the data sent by the gRPC server, see the [/web directory](web)
