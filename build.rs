use std::path::Path;
use std::{env, fs};

fn main() {
    let pwd = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR env variable unset");
    let dir = Path::new(&pwd).join("proto/hdfs");
    let files = fs::read_dir(&dir)
        .expect("read dir succeed")
        .map(|v| dir.join(v.expect("read entry succeed").path()))
        .collect::<Vec<_>>();

    for v in &files {
        println!("cargo:rerun-if-changed={}", v.to_string_lossy());
    }

    prost_build::compile_protos(
        &files,
        &[
            Path::new(&pwd).join("proto/common"),
            Path::new(&pwd).join("proto/hdfs"),
        ],
    )
    .unwrap();
}
