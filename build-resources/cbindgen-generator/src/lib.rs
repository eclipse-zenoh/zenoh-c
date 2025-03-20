mod cbindgen;
mod splitguide;

pub use cbindgen::generate_c_headers;
pub(crate) use splitguide::{split_bindings, FuncArg, FunctionSignature};
