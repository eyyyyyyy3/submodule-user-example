use std::error::Error;

use proto::example_server::{Example, ExampleServer};
use tonic::{transport::Server, Response};

mod proto {
    tonic::include_proto!("example.v1");
}

#[derive(Default, Debug)]
struct ExampleService {}

#[tonic::async_trait]
impl Example for ExampleService {
    async fn say_something(
        &self,
        _request: tonic::Request<proto::SaySomethingRequest>,
    ) -> Result<tonic::Response<proto::SaySomethingResponse>, tonic::Status> {
        let response = proto::SaySomethingResponse {
            answer: "HELLO FROM THE SERVER".to_owned(),
        };

        Ok(tonic::Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:4000".parse()?;

    let example_service = ExampleService::default();
    let example_server = ExampleServer::new(example_service);

    Server::builder()
        .add_service(example_server)
        .serve(addr)
        .await?;
    Ok(())
}
