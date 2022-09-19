#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init();
    visualizer_server::run::http_server(8050).await
}
