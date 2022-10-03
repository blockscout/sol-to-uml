use actix_prost_build::{ActixGenerator, GeneratorList};
use prost_build::{Config, ServiceGenerator};
use std::path::Path;

fn option_base64() -> &'static str {
    "#[serde_as(as = \"Option<serde_with::base64::Base64>\")]"
}

// custom function to include custom generator
fn compile(
    protos: &[impl AsRef<Path>],
    includes: &[impl AsRef<Path>],
    generator: Box<dyn ServiceGenerator>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Config::new();
    config
        .service_generator(generator)
        .compile_well_known_types()
        .protoc_arg("--openapiv2_out=proto")
        .protoc_arg("--openapiv2_opt")
        .protoc_arg("grpc_api_configuration=proto/api_config_http.yaml,output_format=yaml,allow_merge=true,merge_file_name=visualizer")
        // NOTE: order is matter. serde_with should be before serde::Serialize
        .type_attribute(".", "#[serde_with::serde_as]")
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .field_attribute(
            ".blockscout.visualizer.v1.VisualizeResponse.png",
            option_base64(),
        )
        .field_attribute(
            ".blockscout.visualizer.v1.VisualizeResponse.svg",
            option_base64(),
        );
    config.compile_protos(protos, includes)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gens = Box::new(GeneratorList::new(vec![
        tonic_build::configure().service_generator(),
        Box::new(ActixGenerator::new("proto/api_config_http.yaml").unwrap()),
    ]));
    compile(
        &["proto/visualizer.proto", "proto/health.proto"],
        &["proto"],
        gens,
    )?;
    Ok(())
}
