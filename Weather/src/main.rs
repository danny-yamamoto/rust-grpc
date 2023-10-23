use tonic::{Request, Response, Status};
use tonic_reflection::server::Builder;

mod weather {tonic::include_proto!("weather.v1");}

use weather::{
    weather_service_server::{WeatherService, WeatherServiceServer}, 
    WeatherRequest, WeatherResponse,
};

pub struct StructWeatherService {}

#[tonic::async_trait]
impl WeatherService for StructWeatherService {
    async fn weather(&self, request: Request<WeatherRequest>) -> Result<Response<WeatherResponse>, Status> {
        let res = WeatherResponse {text: format!("Today's weather is {}!", request.into_inner().condition),};
        Ok(Response::new(res))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:8080".parse()?;
    let weather_service = StructWeatherService {};
    tonic::transport::Server::builder()
        .add_service(WeatherServiceServer::new(weather_service))
        .add_service(
            Builder::configure()
                .register_encoded_file_descriptor_set(tonic::include_file_descriptor_set!(
                    "weather_descriptor"
                ))
                .build()
                .unwrap(),
        )
        .serve(addr)
        .await?;

    Ok(())
}
