mod proto;

fn main() {
    dbg!(
        proto::blockscout::visualizer::v1::VisualizeContractsRequest {
            sources: Default::default(),
            output_mask: Default::default()
        }
    );
}
