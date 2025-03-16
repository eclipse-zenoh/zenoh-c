mod cbindgen;
mod splitguide;
mod opaque_types;
mod utils;

pub use opaque_types::generate_opaque_types;
pub use cbindgen::generate_c_headers;
pub use utils::get_build_rs_path;
pub use utils::split_type_name;
pub use utils::test_feature;
pub use utils::FEATURES;
pub use utils::cargo_target_dir;
pub use splitguide::split_bindings;
pub use splitguide::FuncArg;
pub use splitguide::FunctionSignature;