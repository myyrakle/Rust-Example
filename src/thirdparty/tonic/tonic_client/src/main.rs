pub mod tutorial {
    tonic::include_proto!("tutorial");
}

use tutorial::foo_client::FooClient;
use tutorial::{AddReply, AddRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = FooClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(AddRequest { lhs: 10, rhs: 20 });

    let response: tonic::Response<AddReply> = client.add(request).await?;

    println!("RESPONSE={:?}", response.into_inner());

    Ok(())






    
}
