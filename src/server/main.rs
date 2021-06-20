use grpc::{ServerHandlerContext, ServerRequestSingle, ServerResponseUnarySink};
// importing generated gRPC code
use pong_grpc::*;
// importing types for messages
use pong::*;
mod pong;
mod pong_grpc;

struct MySay;
impl Say for MySay {
    // rpc for service
    fn send(
        &self,
        _: ServerHandlerContext,
        req: ServerRequestSingle<SayRequest>,
        resp: ServerResponseUnarySink<SayResponse>,
    ) -> grpc::Result<()> {
        // create Response
        let mut r = SayResponse::new();
        let name = if req.message.get_name().is_empty() {
            "world"
        } else {
            req.message.get_name()
        };
        // sent the response
        println!("greeting request from {}", name);
        r.set_message(format!("Hello {}", name));
        resp.finish(r)
    }
}

fn main() {
    let port = 50051;
    // creating server
    let mut server = grpc::ServerBuilder::new_plain();
    // adding port to server for http
    server.http.set_port(port);
    // adding say service to server
    server.add_service(SayServer::new_service_def(MySay));
    // running the server
    let _server = server.build().expect("server");
    println!("greeter server started on port {}", port,);
    // stopping the program from finishing
    loop {
        std::thread::park();
    }
}
