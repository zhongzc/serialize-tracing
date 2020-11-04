use flatc_rust;
use protobuf_build::Builder;

use std::path::Path;

fn main() {
    Builder::new().search_dir_for_protos("proto").generate();

    println!("cargo:rerun-if-changed=fbs/tracing.fbs");
    flatc_rust::run(flatc_rust::Args {
        inputs: &[Path::new("fbs/tracing.fbs")],
        out_dir: Path::new(&format!(
            "{}/flatbuffers",
            std::env::var("OUT_DIR").expect("No OUT_DIR defined")
        )),
        ..Default::default()
    })
    .expect("flatc");
}
