use actix_web::{dev::Server, App, HttpServer};
use actix_web_prom::{PrometheusMetrics, PrometheusMetricsBuilder};
use lazy_static::lazy_static;
use prometheus::{
    register_histogram_vec_with_registry, register_histogram_with_registry,
    register_int_counter_vec_with_registry, Histogram, HistogramVec, IntCounterVec, Registry,
};
use std::net::SocketAddr;

lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();
    pub static ref SOL2UML: IntCounterVec = register_int_counter_vec_with_registry!(
        "sol_to_uml_so2uml_requests",
        "number of uml and storage requests",
        &["endpoint", "status"],
        REGISTRY,
    )
    .unwrap();
    pub static ref SAVEFILES_TIME: Histogram = register_histogram_with_registry!(
        "sol_to_uml_save_files_time",
        "running time for saving files with contracts",
        vec![0.01, 0.025, 0.05, 0.075, 0.1, 0.25, 0.5, 0.75, 1.0, 2.5, 5.0, 7.5, 10.0, 20.0],
        REGISTRY,
    )
    .unwrap();
    pub static ref SOL2UML_RUN_TIME: HistogramVec = register_histogram_vec_with_registry!(
        "sol_to_uml_sol2uml_time",
        "running time for sol2uml library",
        &["type"],
        vec![0.01, 0.025, 0.05, 0.075, 0.1, 0.25, 0.5, 0.75, 1.0, 2.5, 5.0, 7.5, 10.0, 20.0],
        REGISTRY,
    )
    .unwrap();
}

pub fn count_sol2uml_request(status: bool, method: &str) {
    let status = match status {
        true => "ok",
        false => "fail",
    };
    SOL2UML.with_label_values(&[method, status]).inc();
}

#[derive(Clone)]
pub struct Metrics {
    metrics_middleware: PrometheusMetrics,
    sol_to_uml_middleware: PrometheusMetrics,
}

impl Metrics {
    pub fn new(endpoint: String) -> Self {
        let metrics_middleware = PrometheusMetricsBuilder::new("sol_to_uml_metrics")
            .registry(REGISTRY.clone())
            .endpoint(&endpoint)
            .build()
            .unwrap();
        // note: sol_to_uml middleware has no endpoint
        let sol_to_uml_middleware = PrometheusMetricsBuilder::new("sol_to_uml")
            .registry(REGISTRY.clone())
            .build()
            .unwrap();

        Self {
            metrics_middleware,
            sol_to_uml_middleware,
        }
    }

    pub fn middleware(&self) -> &PrometheusMetrics {
        &self.sol_to_uml_middleware
    }

    pub fn run_server(&self, addr: SocketAddr) -> Server {
        let metrics_middleware = self.metrics_middleware.clone();
        HttpServer::new(move || App::new().wrap(metrics_middleware.clone()))
            .bind(addr)
            .unwrap()
            .run()
    }
}
