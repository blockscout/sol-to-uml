use futures::try_join;


#[actix_web::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::init();
    let http_server = visualizer_server::run::http_server(8050);
    let grpc_server = visualizer_server::run::grpc_server(8051);

    let (_, _) = try_join!(http_server, grpc_server)?;

    Ok(())
}
