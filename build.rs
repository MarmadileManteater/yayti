
use std::fs::copy;
use prost_build;

fn main() {
  prost_build::compile_protos(&[
    "src/proto/playlist_continuations.proto",
    "src/proto/visitor_data.proto"
  ], &["src/"]).unwrap();
  let out_dir = std::env::var("OUT_DIR").unwrap();
  copy(format!("{}/yayti.items.rs", out_dir), "./src/proto.rs").unwrap();
}
