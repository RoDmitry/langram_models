use include_dir::{include_dir, Dir};

pub const MODELS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/models");
