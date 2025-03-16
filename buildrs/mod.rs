mod cbindgen;
mod opaque_types;
mod splitguide;

pub use cbindgen::generate_c_headers;
pub use opaque_types::generate_opaque_types;
pub use splitguide::{split_bindings, FuncArg, FunctionSignature};
