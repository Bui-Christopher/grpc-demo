use proto::coffees::coffee_client::CoffeeClient;
use proto::coffees::CreateCoffeeRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = CoffeeClient::connect("http://[::1]:50051").await?;

    let req = tonic::Request::new(
        CreateCoffeeRequest {
            person_name: "Chris".to_string(),
            coffee_type: "Vanilla Latte".to_string(),
        }
    );

    let response = client.create_coffee(req).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
