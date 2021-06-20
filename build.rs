fn main() {
    // compile protocol buffer using protoc
    protoc_rust_grpc::Codegen::new()
        .out_dir("src/server")
        .input("./pong.proto")
        .rust_protobuf(true)
        .run()
        .expect("error compiling protocol buffer");
        
    // compile protocol buffer using protoc
    protoc_rust_grpc::Codegen::new()
        .out_dir("src/client/systems/network")
        .input("./pong.proto")
        .rust_protobuf(true)
        .run()
        .expect("error compiling protocol buffer");
}
