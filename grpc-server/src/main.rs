use tonic::{transport::Server, Request, Response, Status};

use proto::coffees::coffee_server::{Coffee, CoffeeServer};
use proto::coffees::{CreateCoffeeRequest, CreateCoffeeResponse};

#[derive(Debug, Default)]
pub struct CoffeeService {}

#[tonic::async_trait]
impl Coffee for CoffeeService {
    async fn create_coffee(
        &self,
        request: Request<CreateCoffeeRequest>,
    ) -> Result<Response<CreateCoffeeResponse>, Status> {
        println!("Request={:?}", request);
        let req = request.into_inner();
        let resp = CreateCoffeeResponse {
            successful: true,
            message: format!("Barista has created a {}", req.coffee_type),
        };
        Ok(Response::new(resp))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let coffee_service = CoffeeService::default();

    Server::builder()
        .add_service(CoffeeServer::new(coffee_service))
        .serve(addr)
        .await?;

    Ok(())
}
