pub mod hello {
    tonic::include_proto!("hello");
}

use hello::greeter_server::{Greeter, GreeterServer};
use hello::{HelloReply, HelloRequest};
use tonic::{Request, Response, Status, transport::Server};

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let request = request.into_inner();
        println!("Received name {}", request.name);
        let reply = HelloReply {
            message: format!("Hello, {}!", request.name),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
