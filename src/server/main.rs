use grpc::{ServerHandlerContext, ServerRequestSingle, ServerResponseUnarySink};
// importing generated gRPC code
use pong_grpc::*;
// importing types for messages
use pong::*;
mod pong;
mod pong_grpc;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Default)]
struct MySay {
    book_reviews: Mutex<HashMap<String, f32>>, 
    book_reviews2: Mutex<Option<String>>, 
    book_reviews3: Mutex<(f32, f32)>, 
}
impl Say for MySay {
    // rpc for service
    fn send(
        &self,
        _: ServerHandlerContext,
        req: ServerRequestSingle<SayRequest>,
        resp: ServerResponseUnarySink<SayResponse>,
    ) -> grpc::Result<()> {
        // create Response
        let mut book_reviews = self.book_reviews.lock().unwrap();
        let name = req.message.get_name();
        *book_reviews.entry(name.to_string()).or_insert(req.message.get_networkname()) = req.message.get_networkname();

        let mut book_reviews2 = self.book_reviews2.lock().unwrap();
        if *book_reviews2 == None {
            *book_reviews2 = Option::from(req.message.get_name().to_string())
        }

        let is_host = book_reviews2.as_ref().unwrap() == req.message.get_name();
        if is_host {
            let mut book_reviews3 = self.book_reviews3.lock().unwrap();
            *book_reviews3 = (req.message.get_networkname2(), req.message.get_networkname3());
        }

        let mut r = SayResponse::new();
        // sent the response
        if let Some(a) = book_reviews.iter()
            .filter(|(k, _)| *k != req.message.get_name())
            .next() {
                let (_, b) = a;               
                r.set_message(*b);
            }
        r.set_message2(self.book_reviews3.lock().unwrap().0);
        let mut x = self.book_reviews3.lock().unwrap().1;
        if !is_host {
            x = 100.0 - x;
        }
        r.set_message3(x);
        println!("{} {} {}", req.message.get_name(), req.message.get_networkname(), r.get_message());
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
    server.add_service(SayServer::new_service_def(MySay::default()));
    // running the server
    let _server = server.build().expect("server");
    println!("greeter server started on port {}", port,);
    // stopping the program from finishing
    loop {
        std::thread::park();
    }
}
