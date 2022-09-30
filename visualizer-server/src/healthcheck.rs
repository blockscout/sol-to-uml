use crate::proto::blockscout::visualizer::v1::{
    health_check_response, health_check_server::HealthCheck, HealthCheckRequest,
    HealthCheckResponse,
};

pub use crate::proto::blockscout::visualizer::v1::health_check_actix::route_health_check;

#[derive(Default)]
pub struct HealthCheckService {}

#[async_trait::async_trait]
impl HealthCheck for HealthCheckService {
    async fn check(
        &self,
        _request: tonic::Request<HealthCheckRequest>,
    ) -> Result<tonic::Response<HealthCheckResponse>, tonic::Status> {
        Ok(tonic::Response::new(HealthCheckResponse {
            status: health_check_response::ServingStatus::Alive as i32,
        }))
    }
}
