pub mod tutorial {
    tonic::include_proto!("tutorial");
}

use tonic::{transport::Server, Request, Response, Status};

use tutorial::foo_server::{Foo, FooServer};
use tutorial::{AddReply, AddRequest};

#[derive(Default)]
pub struct TestServer {}

#[tonic::async_trait]
impl Foo for TestServer {
    async fn add(&self, request: Request<AddRequest>) -> Result<Response<AddReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let request = request.into_inner();
        let lhs = request.lhs;
        let rhs = request.rhs;

        let result = lhs + rhs;

        println!("덧셈 처리: {} + {} = {}", lhs, rhs, result);

        let reply = AddReply { result: result };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let test_server = TestServer::default();

    println!("Test Server listening on {}", addr);

    Server::builder()
        .add_service(FooServer::new(test_server))
        .serve(addr)
        .await?;

    Ok(())
}
