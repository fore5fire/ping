use std::env;
use std::sync::Arc;

// importing generated gRPC code
use pong_grpc::*;
// importing types for messages
use pong::*;
mod pong;
mod pong_grpc;

use futures::executor;
use grpc::ClientStub;
use grpc::ClientStubExt;

fn main() {
    let name = "anshul";
    let port = 50051;
    let client_conf = Default::default();
    // create a client
    let client = SayClient::new_plain("::1", port, client_conf).unwrap();
    // create request
    let mut req = SayRequest::new();
    req.set_name(name.to_string());
    // send the request
    let resp = client
        .send(grpc::RequestOptions::new(), req)
        .join_metadata_result();
    // wait for response
    println!("{:?}", executor::block_on(resp));
}
